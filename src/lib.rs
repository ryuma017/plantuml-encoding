mod errors;
mod utils;

use flate2::write;

use std::io::Write;

pub use crate::errors::FromPlantumlError;

pub fn encode_plantuml_hex<T: AsRef<str>>(plantuml: T) -> Result<String, FromPlantumlError> {
    let hex = hex::encode(plantuml.as_ref());

    Ok(String::from("~h") + &hex)
}

pub fn decode_plantuml_hex<T: AsRef<str>>(hex: T) -> Result<String, FromPlantumlError> {
    let plantuml_hex_trimmed = hex.as_ref().trim_start_matches("~h");

    let decoded_bytes = hex::decode(plantuml_hex_trimmed)?;

    Ok(String::from_utf8(decoded_bytes)?)
}

pub fn encode_plantuml_deflate<T: AsRef<str>>(plantuml: T) -> Result<String, FromPlantumlError> {
    let mut encoder = write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(plantuml.as_ref().as_bytes())?;

    let encoded_bytes = encoder.finish()?;

    Ok(utils::encode_plantuml_for_deflate(&encoded_bytes))
}

#[allow(clippy::unused_io_amount)]
pub fn decode_plantuml_deflate<T: AsRef<str>>(
    plantuml_deflated: T,
) -> Result<String, errors::FromPlantumlError> {
    let result = match utils::decode_plantuml_for_deflate(plantuml_deflated.as_ref()) {
        Some(r) => r,
        None => {
            return Err(errors::FromPlantumlError(
                "internal decoding error (out of bounds or similar)".to_string(),
            ));
        }
    };

    let mut deflater = write::DeflateDecoder::new(Vec::new());
    for item in result.into_iter() {
        // write_all produces `failed to write whole buffer` issue with some data
        deflater.write(&[item])?;
    }
    let decoded_bytes = deflater.finish()?;

    Ok(String::from_utf8(decoded_bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_plantuml_hex() {
        assert_eq!(
            encode_plantuml_hex("@startuml\nPUML -> RUST: HELLO \n@enduml"),
            Ok(
                "~h407374617274756d6c0a50554d4c202d3e20525553543a2048454c4c4f200a40656e64756d6c"
                    .to_string()
            )
        );
    }

    #[test]
    fn it_decodes_plantuml_hex() {
        assert_eq!(
            decode_plantuml_hex(
                "~h407374617274756d6c0a50554d4c202d3e20525553543a2048454c4c4f200a40656e64756d6c"
            ),
            Ok("@startuml\nPUML -> RUST: HELLO \n@enduml".to_string())
        );
    }

    #[test]
    #[allow(clippy::unnecessary_to_owned)]
    fn it_encodes_plantuml_hex_from_string() {
        assert_eq!(
            encode_plantuml_hex("@startuml\nPUML -> RUST: HELLO \n@enduml".to_string()),
            Ok(
                "~h407374617274756d6c0a50554d4c202d3e20525553543a2048454c4c4f200a40656e64756d6c"
                    .to_string()
            )
        );
    }

    #[test]
    fn it_decodes_plantuml_hex_error() {
        assert_eq!(
            decode_plantuml_hex("12345"),
            Err(FromPlantumlError(
                "there is a problem during hex decoding: `Odd number of digits`".to_string()
            ))
        );
    }

    #[test]
    fn it_encodes_plantuml_deflate() {
        assert_eq!(
            encode_plantuml_deflate("@startuml\nPUML -> RUST: HELLO \n@enduml"),
            Ok("0IO0sVz0StHXSdHrRMmAK5LDJ20jFY1ILLDKEY18HKnCJo0AG6LkP7LjR000".to_string())
        );
    }

    #[test]
    fn it_decodes_plantuml_deflate() {
        assert_eq!(
            decode_plantuml_deflate("0IO0sVz0StHXSdHrRMmAK5LDJ20jFY1ILLDKEY18HKnCJo0AG6LkP7LjR000"),
            Ok("@startuml\nPUML -> RUST: HELLO \n@enduml".to_string())
        );
    }

    #[test]
    fn it_decode_plantuml_deflate_error() {
        assert_eq!(
            decode_plantuml_deflate("4444"),
            Err(FromPlantumlError(
                "there is a problem during deflate decoding: `deflate decompression error`"
                    .to_string()
            ))
        );
    }
}
