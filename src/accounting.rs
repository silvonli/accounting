

pub struct Accounting {
    // currency symbol (default: $)
	symbol: String,
    // currency precision (default: 0)
	precision: usize,  
    // thousand separator (default: ,)  
	thousand: String,
    // decimal separator (default: .)
	decimal: String, 
    // format string allows control of symbol (default: {s}{v})
	format: String,
    // format string for negative values (default: -{s}{v})
	format_negative: String,
    // format string for zero values (default: {s}{v})
	format_zero: String
}

impl Default for Accounting {
    fn default() -> Self {
        let format = "{s}{v}";
        Accounting {
            symbol: "$".to_string(), 
            precision: 0, 
            thousand: ",".to_string(),
            decimal: ".".to_string(), 
            format: format.to_string(), 
            format_negative: "-".to_string() + format, 
            format_zero: format.to_string()
        }
    }
}

impl Accounting {

    fn new(
        symbol: &str, 
        precision: usize, 
        thousand: &str, 
        decimal: &str, 
        format: &str, 
        format_negative: &str, 
        format_zero: &str
    ) -> Self {
        Accounting {
            symbol: symbol.to_string(), 
            precision, 
            thousand: thousand.to_string(), 
            decimal: decimal.to_string(), 
            format: format.to_string(), 
            format_negative: format_negative.to_string(), 
            format_zero: format_zero.to_string()
        }
    }

    // sets the separator for the thousands separation
    pub fn set_thousand_separator(&mut self, str: &str) {
        self.thousand = str.to_string();
    }

    // sets the separator for the decimal separation
    pub fn set_decimal_separator(&mut self, str: &str) {
        self.decimal = str.to_string();
    }

    // sets the Format default: {s}{v} ({s}=Symbol;{v}=Value)
    pub fn set_format(&mut self, str: &str) {
        self.format = str.to_string();
    }

    // sets the Format for negative values default: -{s}{v} 
    pub fn set_format_negative(&mut self, str: &str) {
        self.format_negative = str.to_string();
    }

    // sets the Format for zero values default: {s}{v} 
    pub fn set_format_zero(&mut self, str: &str) {
        self.format_zero = str.to_string();
    }

}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn accounting_test() {
        let accounting = Accounting{symbol:"$",precision:2, ..Default::default()};
        accounting.set_format("{v} {s}");
        assert_eq!(accounting.FormatMoney(123456789.213123), "123,456,789.21 $");
	}
}