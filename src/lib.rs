#[cfg(test)]
mod tests {
    #[test]
    fn it_encodes_plantuml_hex() {
        assert_eq!(
        encodes_plantuml_hex("&startuml\nPUML -> RUST: HELLO \n@enduml"),
        ""
        );
    }
}
