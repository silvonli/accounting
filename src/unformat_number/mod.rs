
use regex::Regex;

mod locale;

// unformat_number takes a string of the number to strip currency info on
// and precision for decimals.
// It pulls the currency descripter from the locale_info_map and uses it to return an unformatted value
// based on thousands seperator and decimal seperator 
pub fn unformat_number(n: &str, precision: usize, currency: &str) -> String {
	let lc: locale::Locale;
	let currency = currency.to_uppercase();
	
	if let Some(val)=  locale::locale_info_map(&currency) {
		lc = val;
	} else {
		panic!("No Locale Info Found");
	}

	let r = Regex::new(r"[^0-9-., ]").unwrap();
	let mut num = r.replace_all(n, "").into_owned();
	num = num.replace(lc.thousands_seperator, "");

	// Replace decimal seperator with a decimal at specified precision
	if lc.decimal_seperator != "." {
		num = num.replace(lc.decimal_seperator, ".");
	}

	num = set_precision(&num, precision);
	num
}

fn set_precision(num: &str, precision: usize) -> String {
	let v: f64 = num.trim().parse().unwrap();
	format!("{0:.1$}", v, precision)
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn unformat_number_test() {
		assert_eq!(unformat_number("$4,500.23", 2, "USD"), "4500.23");
		assert_eq!(unformat_number("EUR 45.000,33", 2, "eur"), "45000.33");
		assert_eq!(unformat_number("EUR 111.145.000,33", 2, "eur"), "111145000.33");
	}

	#[test]
	fn unformat_number_error_test() {
		// unformat_number("$45,567.10", 2, "zzz");
		// todo
	}
}