pub fn encodes_plantuml_hex(plantuml: &str) -> String {
    let hex = hex::encode(plantuml);

    String::from("~h") + &hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_plantuml_hex() {
        assert_eq!(
        encodes_plantuml_hex("@startuml\nPUML -> RUST: HELLO \n@enduml"),
        "~h407374617274756d6c0a50554d4c202d3e20525553543a2048454c4c4f200a40656e64756d6c"
        );
    }
}
