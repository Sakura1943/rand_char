use super::{cli::Cli, result::Result};
use rand::Rng;

pub fn gen_char(
    Cli {
        length,
        count,
        ignore,
        ignore_symbol,
        only_lowercase,
        only_letter,
        only_uppercase,
        only_number,
        ..
    }: Cli,
) -> Result<Vec<String>> {
    let mut chars = Vec::new();
    let mut charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~+-"
        .as_bytes();
    if ignore {
        charset = "ABCDEFGHJKMNPQRSTUVWXYZ\
                            abcdefghjkmnpqrstuvwxyz\
                            23456789*&^%$#@~+-"
            .as_bytes();
    }
    if ignore_symbol {
        charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789"
            .as_bytes();
        if ignore {
            charset = "ABCDEFGHJKMNPQRSTUVWXYZ\
                            abcdefghjkmnpqrstuvwxyz\
                            23456789"
                .as_bytes();
        }
    }
    if only_number {
        charset = "0123456789".as_bytes();
        if ignore {
            charset = "23456789".as_bytes();
        }
    }
    if only_uppercase {
        charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
        if ignore {
            charset = "ABCDEFGHJKMNPQRSTUVWXYZ".as_bytes();
        }
    }
    if only_letter {
        charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz"
            .as_bytes();
        if ignore {
            charset = "ABCDEFGHJKMNPQRSTUVWXYZ\
                abcdefghjkmnpqrstuvwxyz"
                .as_bytes();
        }
    }
    if only_lowercase {
        charset = "abcdefghijklmnopqrstuvwxyz".as_bytes();
        if ignore {
            charset = "abcdefghjkmnpqrstuvwxyz".as_bytes();
        }
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
