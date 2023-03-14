use rand::Rng;

use super::result::Result;

pub fn gen_char(length: u8, count: u8, ignore: bool) -> Result<Vec<String>> {
    let mut chars = Vec::new();
    let mut charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~+-"
        .as_bytes();
    if ignore {
        charset = "ABCDEFGHJKMNPQRSTUVWXYZ\
                            abcdefghjkmnpqrstuvwxyz\
                            23456789)(*&^%$#@!~+-"
            .as_bytes();
    }
    for _ in 0..count {
        chars.push(
            (0..length)
                .map(|_| {
                    let index = rand::thread_rng().gen_range(0..charset.len());
                    charset[index] as char
                })
                .collect::<String>(),
        );
    }
    Ok(chars)
}
