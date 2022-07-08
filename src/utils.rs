fn encode_6_bit(mut b: u8) -> String {
    if b < 10 {
        return String::from((48 + b) as char);
    }
    b -= 10;

    if b < 26 {
        return String::from((65 + b) as char);
    }
    b -= 26;

    if b < 26 {
        return String::from((97 + b) as char);
    }
    b -= 26;

    if b == 0 {
        return String::from("-");
    }

    if b == 1 {
        return String::from("_");
    }

    String::from("?")
}

fn append_3_bytes(b1: &u8, b2: &u8, b3: &u8) -> String {
    let c1 = b1 >> 2;
    let c2 = ((b1 & 0x3) << 4) | (b2 >> 4);
    let c3 = ((b2 & 0xF) << 2) | (b3 >> 6);
    let c4 = b3 & 0x3F;

    let mut result = String::new();

    result += &encode_6_bit(c1 & 0x3F);
    result += &encode_6_bit(c2 & 0x3F);
    result += &encode_6_bit(c3 & 0x3F);
    result += &encode_6_bit(c4 & 0x3F);

    result
}

// delfate encoding のための hex encoding とは異なり
// plantumlサーバーは plantuml 文字列の単なる圧縮バージョンではなく、 特別なエンコーディング文字列を要求する
pub fn encode_plantuml_for_deflate(encoded_bytes: &[u8]) -> String {
    let mut result = String::new();

    for (index, byte) in encoded_bytes.iter().enumerate().step_by(3) {
        if index + 2 == encoded_bytes.len() {
            result += &append_3_bytes(byte, &encoded_bytes[index + 1], &0);
            continue;
        }

        if index + 1 == encoded_bytes.len() {
            result += &append_3_bytes(byte, &0, &0);
            continue;
        }

        result += &append_3_bytes(byte, &encoded_bytes[index + 1], &encoded_bytes[index + 2]);
    }

    result
}
