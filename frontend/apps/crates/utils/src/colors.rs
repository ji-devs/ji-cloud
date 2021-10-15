use crate::unwrap::UnwrapJiExt;
use rgb::RGBA8;

pub fn hex_to_rgba8(hex: &str) -> RGBA8 {
    let r = u8::from_str_radix(&hex[1..=2], 16).expect_ji("Invalid color");
    let g = u8::from_str_radix(&hex[3..=4], 16).expect_ji("Invalid color");
    let b = u8::from_str_radix(&hex[5..=6], 16).expect_ji("Invalid color");
    let a = if hex.len() > 7 {
        u8::from_str_radix(&hex[7..=8], 16).expect_ji("Invalid color")
    } else {
        0xFF
    };

    RGBA8::new(r, g, b, a)
}

pub fn rgba8_to_hex(rgba8: &RGBA8) -> String {
    format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        rgba8.r, rgba8.g, rgba8.b, rgba8.a
    )
}
