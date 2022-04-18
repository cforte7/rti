use regex::Regex;


pub mod regex_conversion {
    struct RegexConversion {
        regex_checker: regex::Regex,
        parse_pattern: Vec[String],
    }

    let slash_date_regex: RegexConversion = Regex::new(r"^[0,1]?\d{1}\/(([0-2]?\d{1})|([3][0,1]{1}))\/(([1]{1}[9]{1}[9]{1}\d{1})|([2-9]{1}\d{3}))$",).unwrap();
    let slash_date_patterns = ""

    let ampm_time_regex = Regex::new(r"^(0?[1-9]|1[0-2]):([0-5]\d)\s?((?:[Aa]|[Pp])\.?[Mm]\.?)$")
    
    // upper and lower case AM/PM
    let time_patterns = [
        "%I:%M %P",  // 01:23 PM
        "%I:%M %p" // 01:23 pm
        "%l:%M %P" // 1:23 PM
        "%l:%M %p" // 1:23 pm
        "%H:%M" // 
        "%Y-%m-%d %H:%M:%S" // 2015-09-05 23:56:04
    ]
    
}
