

pub struct Locale<'a> {
	// currency name
	pub name: &'a str,
	// default decimal length
	pub fraction_length: usize,
	// thousands seperator
	pub thousands_seperator: &'a str,
	// decimal seperator
	pub decimal_seperator: &'a str,
	// space seperator
	pub space_seperator: &'a str,
	// UTF symbol
	pub utf_symbol: &'a str,
	// HTML symbol
	pub html_symbol: &'a str,
	// Common symbol
	pub common_symbol: &'a str,
	// symbol before or after currency
	pub is_pre: bool,
}

impl <'a> Locale <'a> {
	pub fn new(
		name: &'a str,
		fraction_length: usize,
		thousands_seperator: &'a str,
		decimal_seperator: &'a str,
		space_seperator: &'a str,
		utf_symbol: &'a str,
		html_symbol: &'a str,
		common_symbol: &'a str,
		is_pre: bool
	) -> Self {
		Locale{name, fraction_length, thousands_seperator, decimal_seperator, 
			space_seperator, utf_symbol, html_symbol, common_symbol, is_pre}
	}
}

pub fn locale_map(currency: &str) -> Option<Locale> {
	let empty = "";
    match currency {
		"AED" => Some(Locale::new("UAE Dirham", 2, ",", ".", " ", empty, empty, "Dhs.", true)),
		"AFA" => Some(Locale::new("Afghani", 0, empty, empty, "060B", "&#x060B;", empty, "؋", true)),
		"ALL" => Some(Locale::new("Lek", 2, empty, empty, "", empty, empty, "Lek", true)),
		"AMD" => Some(Locale::new("Armenian Dram", 2, ",", ".", "", empty, empty, "֏", false)),
		"ANG" => Some(Locale::new("Antillian Guilder", 2, ".", ",", " ", "0192", "&#x0192;", "ƒ", true)),
		"AOA" => Some(Locale::new("New Kwanza", 0, empty, empty, "", empty, empty, "Kz", true)),
		"ARS" => Some(Locale::new("Argentine Peso", 2, ".", ",", "", "20B1", "&#x20B1;", "$", true)),
		"ATS" => Some(Locale::new("Schilling", 2, ".", ",", " ", empty, empty, "öS", true)),
		"AUD" => Some(Locale::new("Australian Dollar", 2, " ", ".", "", "0024", "&#x0024;", "$", true)),
		"AWG" => Some(Locale::new("Aruban Guilder", 2, ",", ".", " ", "0192", "&#x0192;", "ƒ", true)),
		"AZN" => Some(Locale::new("Azerbaijanian Manat", 2, empty, empty, "", empty, empty, "₼", true)),
		"BAM" => Some(Locale::new("Convertible Marks", 2, ",", ".", "", empty, empty, "KM", false)),
		"BBD" => Some(Locale::new("Barbados Dollar", 2, empty, empty, "", "0024", "&#x0024;", "Bds$", true)),
		"BDT" => Some(Locale::new("Taka", 2, ",", ".", " ", empty, empty, "Tk", true)),
		"BEF" => Some(Locale::new("Belgian Franc", 0, ".", "", " ", "20A3", "&#x20A3;", "BEF", true)),
		"BGN" => Some(Locale::new("Lev", 2, " ", ",", " ", empty, empty, "лв", false)),
		"BHD" => Some(Locale::new("Bahraini Dinar", 3, ",", ".", " ", empty, empty, "د.ب", true)),
		"BIF" => Some(Locale::new("Burundi Franc", 0, empty, empty, "", empty, empty, "FBu", true)),
		"BMD" => Some(Locale::new("Bermudian Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"BND" => Some(Locale::new("Brunei Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"BOB" => Some(Locale::new("Bolivian Boliviano", 2, ",", ".", "", empty, empty, "$b", true)),
		"BRL" => Some(Locale::new("Brazilian Real", 2, ".", ",", " ", "0052 0024", "R$", "R$", true)),
		"BSD" => Some(Locale::new("Bahamian Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"BTN" => Some(Locale::new("Bhutan Ngultrum", 2, empty, empty, "", empty, empty, "BTN", true)),
		"BWP" => Some(Locale::new("Pula", 2, ",", ".", "", empty, empty, "P", true)),
		"BYR" => Some(Locale::new("Belarussian Ruble", 0, empty, empty, "", empty, empty, "p.", true)),
		"BZD" => Some(Locale::new("Belize Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"CAD" => Some(Locale::new("Canadian Dollar", 2, ",", ".", "", "0024", "&#x0024;", "CA$", true)),
		"CDF" => Some(Locale::new("Franc Congolais", 2, empty, empty, "", empty, empty, "FC", true)),
		"CHF" => Some(Locale::new("Swiss Franc", 2, "'", ".", " ", empty, empty, "CHF", true)),
		"CLP" => Some(Locale::new("Chilean Peso", 0, ".", "", "", "20B1", "&#x20B1;", "$", true)),
		"CNY" => Some(Locale::new("Yuan Renminbi", 2, ",", ".", "", "5713", "&#x5713;", "¥", true)),
		"COP" => Some(Locale::new("Colombian Peso", 2, ".", ",", "", "20B1", "&#x20B1;", "$", true)),
		"CRC" => Some(Locale::new("Costa Rican Colon", 2, ".", ",", " ", "20A1", "&#x20A1;", "₡", true)),
		"CUP" => Some(Locale::new("Cuban Peso", 2, ",", ".", " ", "20B1", "&#x20B1;", "$", true)),
		"CVE" => Some(Locale::new("Cape Verde Escudo", 0, empty, empty, "", empty, empty, "$", true)),
		"CYP" => Some(Locale::new("Cyprus Pound", 2, ".", ",", "", "00A3", "&#x00A3;", "£", true)),
		"CZK" => Some(Locale::new("Czech Koruna", 2, ".", ",", " ", empty, empty, "Kč", false)),
		"DEM" => Some(Locale::new("Deutsche Mark", 2, ".", ",", "", empty, empty, "DM", false)),
		"DJF" => Some(Locale::new("Djibouti Franc", 0, empty, empty, "", empty, empty, "DJF", true)),
		"DKK" => Some(Locale::new("Danish Krone", 2, ".", ",", "", empty, empty, "kr.", true)),
		"DOP" => Some(Locale::new("Dominican Peso", 2, ",", ".", " ", "20B1", "&#x20B1;", "$", true)),
		"DZD" => Some(Locale::new("Algerian Dinar", 2, empty, empty, "", empty, empty, "DA", true)),
		"ECS" => Some(Locale::new("Sucre", 0, empty, empty, "", empty, empty, "S.", true)),
		"EEK" => Some(Locale::new("Kroon", 2, " ", ",", " ", empty, empty, "kr", false)),
		"EGP" => Some(Locale::new("Egyptian Pound", 2, ",", ".", " ", "00A3", "&#x00A3;", "£", true)),
		"ERN" => Some(Locale::new("Nakfa", 0, empty, empty, "", empty, empty, "NKf", true)),
		"ESP" => Some(Locale::new("Spanish Peseta", 0, ".", "", " ", "20A7", "&#x20A7;", "Ptas", false)),
		"ETB" => Some(Locale::new("Ethiopian Birr", 0, empty, empty, "", empty, empty, "BR", true)),
		"EUR" => Some(Locale::new("Euro", 2, ".", ",", "", "20AC", "&#x20AC;", "€", true)),
		"FIM" => Some(Locale::new("Markka", 2, " ", ",", " ", empty, empty, "mk", false)),
		"FJD" => Some(Locale::new("Fiji Dollar", 0, empty, empty, "", "0024", "&#x0024;", "FJ$", true)),
		"FKP" => Some(Locale::new("Pound", 0, empty, empty, "", "00A3", "&#x00A3;", "£", true)),
		"FRF" => Some(Locale::new("French Franc", 2, " ", ",", " ", "20A3", "&#x20A3;", "Fr", false)),
		"GBP" => Some(Locale::new("Pound Sterling", 2, ",", ".", "", "00A3", "&#x00A3;", "£", true)),
		"GEL" => Some(Locale::new("Lari", 0, empty, empty, "", empty, empty, "GEL", true)),
		"GHS" => Some(Locale::new("Cedi", 2, ",", ".", "", "20B5", "&#x20B5;", "₵", true)),
		"GIP" => Some(Locale::new("Gibraltar Pound", 2, ",", ".", "", "00A3", "&#x00A3;", "£", true)),
		"GMD" => Some(Locale::new("Dalasi", 0, empty, empty, "", empty, empty, "GMD", true)),
		"GNF" => Some(Locale::new("Guinea Franc", 0, empty, empty, empty, empty, empty, "FG", true)),
		"GRD" => Some(Locale::new("Drachma", 2, ".", ",", " ", "20AF", "&#x20AF;", "GRD", false)),
		"GTQ" => Some(Locale::new("Quetzal", 2, ",", ".", "", empty, empty, "Q.", true)),
		"GWP" => Some(Locale::new("Guinea-Bissau Peso", 0, empty, empty, empty, empty, empty, "GWP", true)),
		"GYD" => Some(Locale::new("Guyana Dollar", 0, empty, empty, "", "0024", "&#x0024;", "$", true)),
		"HKD" => Some(Locale::new("Hong Kong Dollar", 2, ",", ".", "", "0024", "&#x0024;", "HK$", true)),
		"HNL" => Some(Locale::new("Lempira", 2, ",", ".", " ", empty, empty, "L", true)),
		"HRK" => Some(Locale::new("Kuna", 2, ".", ",", " ", empty, empty, "kn", false)),
		"HTG" => Some(Locale::new("Gourde", 0, empty, empty, "", empty, empty, "G", true)),
		"HUF" => Some(Locale::new("Forint", 0, ".", "", " ", empty, empty, "Ft", false)),
		"IDR" => Some(Locale::new("Rupiah", 0, ".", ",", "", empty, empty, "Rp", true)),
		"IEP" => Some(Locale::new("Irish Pound", 2, ",", ".", "", "00A3", "&#x00A3;", "£", true)),
		"ILS" => Some(Locale::new("New Israeli Sheqel", 2, ",", ".", " ", "20AA", "&#x20AA;", "₪", false)),
		"INR" => Some(Locale::new("Indian Rupee", 2, ",", ".", "", "20A8", "&#x20A8;", "₹", true)),
		"IQD" => Some(Locale::new("Iraqi Dinar", 3, empty, empty, "", empty, empty, "د.ع", true)),
		"IRR" => Some(Locale::new("Iranian Rial", 2, ",", ".", " ", "FDFC", "&#xFDFC;", "﷼", true)),
		"ISK" => Some(Locale::new("Iceland Krona", 2, ".", ",", " ", empty, empty, "kr", false)),
		"ITL" => Some(Locale::new("Italian Lira", 0, ".", "", " ", "20A4", "&#x20A4;", "L.", true)),
		"JMD" => Some(Locale::new("Jamaican Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"JOD" => Some(Locale::new("Jordanian Dinar", 3, ",", ".", " ", empty, empty, "JD", true)),
		"JPY" => Some(Locale::new("Yen", 0, ",", "", "", "00A5", "&#x00A5;", "¥", true)),
		"KES" => Some(Locale::new("Kenyan Shilling", 2, ",", ".", "", empty, empty, "Ksh", true)),
		"KGS" => Some(Locale::new("Som", 0, empty, empty, "", empty, empty, "лв", true)),
		"KHR" => Some(Locale::new("Riel", 2, empty, empty, "", "17DB", "&#x17DB;", "៛", true)),
		"KMF" => Some(Locale::new("Comoro Franc", 0, empty, empty, "", empty, empty, "KMF", true)),
		"KPW" => Some(Locale::new("North Korean Won", 0, empty, empty, "", "20A9", "&#x20A9;", "₩", true)),
		"KRW" => Some(Locale::new("Won", 0, ",", "", "", "20A9", "&#x20A9;", "₩", true)),
		"KWD" => Some(Locale::new("Kuwaiti Dinar", 3, ",", ".", " ", empty, empty, "ك", true)),
		"KYD" => Some(Locale::new("Cayman Islands Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"KZT" => Some(Locale::new("Tenge", 0, empty, empty, "", empty, empty, "₸", true)),
		"LAK" => Some(Locale::new("Kip", 0, empty, empty, "", "20AD", "&#x20AD;", "₭", true)),
		"LBP" => Some(Locale::new("Lebanese Pound", 0, " ", "", "", "00A3", "&#x00A3;", "ل.ل", false)),
		"LKR" => Some(Locale::new("Sri Lanka Rupee", 0, empty, empty, "", "0BF9", "&#x0BF9;", "₨", true)),
		"LRD" => Some(Locale::new("Liberian Dollar", 0, empty, empty, "", "0024", "&#x0024;", "$", true)),
		"LSL" => Some(Locale::new("Lesotho Maloti", 0, empty, empty, "", empty, empty, "LSL", true)),
		"LTL" => Some(Locale::new("Lithuanian Litas", 2, " ", ",", " ", empty, empty, "Lt", false)),
		"LUF" => Some(Locale::new("Luxembourg Franc", 0, "'", "", " ", "20A3", "&#x20A3;", "F", false)),
		"LVL" => Some(Locale::new("Latvian Lats", 2, ",", ".", " ", empty, empty, "Ls", true)),
		"LYD" => Some(Locale::new("Libyan Dinar", 0, empty, empty, "", empty, empty, "LD", true)),
		"MAD" => Some(Locale::new("Moroccan Dirham", 0, empty, empty, "", empty, empty, "MAD", true)),
		"MDL" => Some(Locale::new("Moldovan Leu", 0, empty, empty, "", empty, empty, "MDL", true)),
		"MGF" => Some(Locale::new("Malagasy Franc", 0, empty, empty, "", empty, empty, "MF", true)),
		"MKD" => Some(Locale::new("Denar", 2, ",", ".", " ", empty, empty, "ден", false)),
		"MMK" => Some(Locale::new("Kyat", 0, empty, empty, "", empty, empty, "K", true)),
		"MNT" => Some(Locale::new("Tugrik", 0, empty, empty, "", "20AE", "&#x20AE;", "₮", true)),
		"MOP" => Some(Locale::new("Pataca", 0, empty, empty, "", empty, empty, "MOP$", true)),
		"MRO" => Some(Locale::new("Ouguiya", 0, empty, empty, "", empty, empty, "MRO", true)),
		"MTL" => Some(Locale::new("Maltese Lira", 2, ",", ".", "", "20A4", "&#x20A4;", "Lm", true)),
		"MUR" => Some(Locale::new("Mauritius Rupee", 0, ",", "", "", "20A8", "&#x20A8;", "Rs", true)),
		"MVR" => Some(Locale::new("Rufiyaa", 0, empty, empty, "", empty, empty, "MVR", true)),
		"MWK" => Some(Locale::new("Kwacha", 2, ",", ".", "", empty, empty, "MK", true)),
		"MXN" => Some(Locale::new("Mexican Peso", 2, ",", ".", " ", "0024", "&#x0024;", "$", true)),
		"MYR" => Some(Locale::new("Malaysian Ringgit", 2, ",", ".", "", empty, empty, "RM", true)),
		"MZN" => Some(Locale::new("Metical", 2, ".", ",", " ", empty, empty, "Mt", false)),
		"NAD" => Some(Locale::new("Namibian Dollar", 0, empty, empty, "", "0024", "&#x0024;", "$", true)),
		"NGN" => Some(Locale::new("Naira", 2, ",", ".", ".", "20A6", "&#x20A6;", "₦", true)),
		"NIO" => Some(Locale::new("Cordoba Oro", 0, empty, empty, "", empty, empty, "C$", true)),
		"NLG" => Some(Locale::new("Netherlands Guilder", 2, ".", ",", " ", "0192", "&#x0192;", "ƒ", true)),
		"NOK" => Some(Locale::new("Norwegian Krone", 2, ".", ",", " ", "kr", "kr", "kr", true)),
		"NPR" => Some(Locale::new("Nepalese Rupee", 2, ",", ".", " ", "20A8", "&#x20A8;", "Rs.", true)),
		"NZD" => Some(Locale::new("New Zealand Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"OMR" => Some(Locale::new("Rial Omani", 3, ",", ".", " ", "FDFC", "&#xFDFC;", "RO", true)),
		"PAB" => Some(Locale::new("Balboa", 0, empty, empty, "", empty, empty, "B/.", true)),
		"PEN" => Some(Locale::new("Nuevo Sol", 2, ",", ".", " ", "S/.", "S/.", "S/.", true)),
		"PGK" => Some(Locale::new("Kina", 0, empty, empty, "", empty, empty, "K", true)),
		"PHP" => Some(Locale::new("Philippine Peso", 2, ",", ".", "", "20B1", "&#x20B1;", "₱", true)),
		"PKR" => Some(Locale::new("Pakistan Rupee", 2, ",", ".", "", "20A8", "&#x20A8;", "Rs", true)),
		"PLN" => Some(Locale::new("Zloty", 2, " ", ",", " ", empty, empty, "zł", false)),
		"PTE" => Some(Locale::new("Portuguese Escudo", 0, ".", "", " ", empty, empty, "$", false)),
		"PYG" => Some(Locale::new("Guarani", 0, empty, empty, "", "20B2", "&#x20B2;", "Gs", true)),
		"QAR" => Some(Locale::new("Qatari Rial", 0, empty, empty, "", "FDFC", "&#xFDFC;", "﷼", true)),
		"RON" => Some(Locale::new("Leu", 2, ".", ",", " ", empty, empty, "lei", false)),
		"RSD" => Some(Locale::new("Serbian Dinar", 2, empty, empty, empty, empty, empty, "РСД", false)),
		"RUB" => Some(Locale::new("Russian Ruble", 2, ".", ",", empty, "0440 0443 0431", "&#x0440;&#x0443;&#x0431;", "₽", true)),
		"RWF" => Some(Locale::new("Rwanda Franc", 0, empty, empty, "", empty, empty, "RWF", true)),
		"SAC" => Some(Locale::new("S. African Rand Commerc.", 0, empty, empty, "", empty, empty, "SAC", true)),
		"SAR" => Some(Locale::new("Saudi Riyal", 2, ",", ".", " ", "FDFC", "&#xFDFC;", "﷼", true)),
		"SBD" => Some(Locale::new("Solomon Islands Dollar", 0, empty, empty, "", "0024", "&#x0024;", "$", true)),
		"SCR" => Some(Locale::new("Seychelles Rupee", 0, empty, empty, "", "20A8", "&#x20A8;", "₨", true)),
		"SDG" => Some(Locale::new("Sudanese Dinar", 0, empty, empty, empty, empty, empty, "LSd", true)),
		"SDP" => Some(Locale::new("Sudanese Pound", 0, empty, empty, "", empty, empty, "SDP", true)),
		"SEK" => Some(Locale::new("Swedish Krona", 2, " ", ",", " ", empty, empty, "kr", false)),
		"SGD" => Some(Locale::new("Singapore Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"SHP" => Some(Locale::new("St Helena Pound", 0, empty, empty, "", "00A3", "&#x00A3;", "£", true)),
		"SIT" => Some(Locale::new("Tolar", 2, ".", ",", " ", empty, empty, "SIT", false)),
		"SKK" => Some(Locale::new("Slovak Koruna", 2, " ", ",", " ", empty, empty, "Sk", false)),
		"SLL" => Some(Locale::new("Leone", 0, empty, empty, "", empty, empty, "Le", true)),
		"SOS" => Some(Locale::new("Somali Shilling", 0, empty, empty, "", empty, empty, "S", true)),
		"SRG" => Some(Locale::new("Surinam Guilder", 0, empty, empty, "", empty, empty, "SRG", true)),
		"STD" => Some(Locale::new("Dobra", 0, empty, empty, "", empty, empty, "DB", true)),
		"SVC" => Some(Locale::new("El Salvador Colon", 2, ",", ".", "", "20A1", "&#x20A1;", "¢", true)),
		"SYP" => Some(Locale::new("Syrian Pound", 0, empty, empty, "", "00A3", "&#x00A3;", "£", true)),
		"SZL" => Some(Locale::new("Lilangeni", 2, "", ".", "", empty, empty, "E", true)),
		"THB" => Some(Locale::new("Baht", 2, ",", ".", " ", "0E3F", "&#x0E3F;", "Bt", false)),
		"TJR" => Some(Locale::new("Tajik Ruble", 0, empty, empty, "", empty, empty, "TJR", true)),
		"TJS" => Some(Locale::new("Somoni", 0, empty, empty, empty, empty, empty, "TJS", true)),
		"TMM" => Some(Locale::new("Manat", 0, empty, empty, "", empty, empty, "T", true)),
		"TND" => Some(Locale::new("Tunisian Dinar", 3, empty, empty, "", empty, empty, "TND", true)),
		"TOP" => Some(Locale::new("Pa'anga", 2, ",", ".", " ", empty, empty, "$", true)),
		"TPE" => Some(Locale::new("Timor Escudo", 0, empty, empty, empty, empty, empty, "TPE", true)),
		"TRY" => Some(Locale::new("Turkish Lira", 0, ",", "", "", "20A4", "&#x20A4;", "TL", false)),
		"TTD" => Some(Locale::new("Trinidad and Tobago Dollar", 0, empty, empty, "", "0024", "&#x0024;", "TT$", true)),
		"TWD" => Some(Locale::new("New Taiwan Dollar", 0, empty, empty, "", "0024", "&#x0024;", "NT$", true)),
		"TZS" => Some(Locale::new("Tanzanian Shilling", 2, ",", ".", " ", empty, empty, "TZs", false)),
		"UAH" => Some(Locale::new("Hryvnia", 2, " ", ",", "", "20B4", "&#x20B4", "UAH", false)),
		"UGX" => Some(Locale::new("Uganda Shilling", 0, empty, empty, "", empty, empty, "UGX", true)),
		"USD" => Some(Locale::new("US Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"UYU" => Some(Locale::new("Peso Uruguayo", 2, ".", ",", "", "20B1", "&#x20B1;", "$", true)),
		"UZS" => Some(Locale::new("Uzbekistan Sum", 0, empty, empty, "", empty, empty, "лв", true)),
		"VEF" => Some(Locale::new("Bolivar", 2, ".", ",", " ", empty, empty, "Bs.", true)),
		"VND" => Some(Locale::new("Dong", 2, ".", ",", " ", "20AB", "&#x20AB;", "₫", true)),
		"VUV" => Some(Locale::new("Vatu", 0, ",", "", "", empty, empty, "VT", false)),
		"WST" => Some(Locale::new("Tala", 0, empty, empty, "", empty, empty, "WST", true)),
		"XAF" => Some(Locale::new("CFA Franc BEAC", 0, empty, empty, "", empty, empty, "$", true)),
		"XCD" => Some(Locale::new("East Caribbean Dollar", 2, ",", ".", "", "0024", "&#x0024;", "$", true)),
		"XOF" => Some(Locale::new("CFA Franc BCEAO", 0, empty, empty, empty, empty, empty, "XOF", true)),
		"XPF" => Some(Locale::new("CFP Franc", 0, empty, empty, "", empty, empty, "XPF", true)),
		"YER" => Some(Locale::new("Yemeni Rial", 0, empty, empty, "", "FDFC", "&#xFDFC;", "﷼", true)),
		"YUN" => Some(Locale::new("New Dinar", 0, empty, empty, "", empty, empty, "YUN", true)),
		"ZAR" => Some(Locale::new("Rand", 2, " ", ".", " ", "0052", "&#x0052;", "R", true)),
		"ZMK" => Some(Locale::new("Kwacha", 0, empty, empty, "", empty, empty, "ZMK", true)),
		"ZRN" => Some(Locale::new("New Zaire", 0, empty, empty, empty, empty, empty, "ZRN", true)),
		"ZWD" => Some(Locale::new("Zimbabwe Dollar ", 2, " ", ".", "", "0024", "&#x0024;", "Z$", true)),
        _   => None,
    }
}


