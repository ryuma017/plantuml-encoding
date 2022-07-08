mod errors;

pub use crate::errors::FromPlantumlError;

pub fn encode_plantuml_hex<T: AsRef<str>>(plantuml: T) -> String {
    let hex = hex::encode(plantuml.as_ref());

    String::from("~h") + &hex
}

pub fn decode_plantuml_hex<T: AsRef<str>>(hex: T) -> Result<String, FromPlantumlError> {
    let plantuml_hex_trimmed = hex.as_ref().trim_start_matches("~h");

    let decoded_bytes = hex::decode(plantuml_hex_trimmed)?;

    Ok(String::from_utf8(decoded_bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_plantuml_hex() {
        assert_eq!(
        encode_plantuml_hex("@startuml\nPUML -> RUST: HELLO \n@enduml"),
        "~h407374617274756d6c0a50554d4c202d3e20525553543a2048454c4c4f200a40656e64756d6c"
        );
    }

    #[test]
    fn it_decodes_plantuml_hex() {
        assert_eq!(
            decode_plantuml_hex("~h407374617274756d6c0a50554d4c202d3e20525553543a2048454c4c4f200a40656e64756d6c"),
            Ok("@startuml\nPUML -> RUST: HELLO \n@enduml".to_string())
        );
    }

    #[test]
    #[allow(clippy::unnecessary_to_owned)]
    fn it_encodes_plantuml_hex_from_string() {
        assert_eq!(
            encode_plantuml_hex("@startuml\nPUML -> RUST: HELLO \n@enduml".to_string()),
            "~h407374617274756d6c0a50554d4c202d3e20525553543a2048454c4c4f200a40656e64756d6c"
        )
    }

    #[test]
    fn it_decodes_plantuml_hex_error() {
        assert_eq!(
            decode_plantuml_hex("12345"),
            Err(errors::FromPlantumlError(
                "there is a problem during hex decoding: `Odd number of digits`".to_string()
            ))
        )
    }
}
