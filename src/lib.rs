pub fn encode_plantuml_hex(plantuml: &str) -> String {
    let hex = hex::encode(plantuml);

    String::from("~h") + &hex
}

pub fn decode_plantuml_hex(hex: &str) -> String {
    let plantuml_hex_trimmed = hex.trim_start_matches("~h");

    let decoded_bytes = hex::decode(plantuml_hex_trimmed).unwrap();

    String::from_utf8(decoded_bytes).unwrap()
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
            "@startuml\nPUML -> RUST: HELLO \n@enduml"
        );
    }
}
