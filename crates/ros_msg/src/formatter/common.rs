

pub static TAB_SIZE: usize = 4;

pub fn build_padding_string(depth: usize) -> String {
    String::from_utf8(vec![b' '; depth]).unwrap()
}
