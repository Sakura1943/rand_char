use super::generate::gen_char;

#[test]
fn test_gen_char() {
   assert!(gen_char(10, 2, false).is_ok());
}
