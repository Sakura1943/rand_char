use super::{generate::gen_char, cli::Cli};

#[test]
fn test_gen_char() {
   assert!(gen_char(Cli::default()).is_ok());
}
