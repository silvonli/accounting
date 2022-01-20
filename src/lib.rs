mod locale;
mod format_macro;
pub mod format_number;
pub mod unformat_number;
pub mod accounting;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
