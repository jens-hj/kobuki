
pub static TAB_SIZE: usize = 4;

pub fn build_padding_string(depth: usize) -> String {
    String::from_utf8(vec![b' '; depth]).unwrap()
}

#[cfg(test)]
mod padding_string {
    use super::build_padding_string;

    #[test]
    fn generates_correct_number_of_padding() {
        assert_eq!("    ", build_padding_string(4))
    }
}