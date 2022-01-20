use crate::format_number::FormatNumber;

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

    pub fn new(
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


    pub fn new_from(symbol: &str, precision: usize) -> Self {
        let mut ac = Self::default();
        ac.symbol = symbol.to_string();
        ac.precision = precision;
        return ac;
    }
    
    pub fn new_from_seperator(symbol: &str, precision: usize, thousand: &str, decimal: &str) -> Self {
        let mut ac = Self::default();
        ac.symbol = symbol.to_string();
        ac.precision = precision;
        ac.thousand = thousand.to_string();
        ac.decimal = decimal.to_string();
        return ac;
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

    // sets the Format of all
    pub fn set_format_all(&mut self, str: &str) {
        self.set_format(str);
        self.set_format_negative(&format!("-{}", str));
        self.set_format_zero(str);
    }
 
    // format_money is a function for formatting numbers as money values,
    // with customisable currency symbol, precision (decimal places), and thousand/decimal separators.
    // supported value types : isize, i8, i16, i32, i64, i128 usize, u8, u16, u32, u64, u128, f32, f64
    pub fn format_money<T:FormatNumber>(&self, value: T) -> String {
        let mut number_string = value.format_number(self.precision, &self.thousand, &self.decimal);
        let zero_string = 0.format_number(self.precision, &self.thousand, &self.decimal);

        let format_string;
        if &number_string[0..1] == "-" {
            number_string = number_string[1..number_string.len()].to_string();
            format_string = &self.format_negative;
        } else if number_string == zero_string {
            format_string = &self.format_zero;
        } else {
            format_string = &self.format;
        }

        let mut result = format_string.replace("{s}", &self.symbol);
        result = result.replace("{v}", &number_string);
        return result;
    }
}