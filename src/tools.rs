/// Generates a an unique name
/// * len - length of the need to generate symbols  // WARNING: maximum value is '32' symbols
pub fn gen_unique_name(len: usize) -> String {
    // checking the length max value:
    if len > 32 { panic!("Maximum value of length is '32' symbols"); }
    // get datetime:
    let datatime = chrono::Utc::now().to_string();
    // generating 'MD5' hash:
    let mut hash = format!("{:x}", md5::compute(datatime));
    // truncating the MD5 hash string:
    hash.truncate(len);
    
    hash
}

/// Converts the text to latin symbols
/// * text - the input text
/// * rm_spaces - the option to remove spaces from string
pub fn to_latin_text(text: &str, rm_spaces: bool) -> String {
    use regex::Regex;

    // replacing non-latin symbols into dash "-":
    let text = Regex::new(&format!(r"[^A-Za-z0-9\-\_{}]+", if !rm_spaces { r"\s" }else{ "" })).unwrap()
        .replace_all(text.trim(), "-");
    
    // removing underscores && dashes from the start && end of string:
    let text = Regex::new(r"(^[_\-]+|[_\-]+$)+").unwrap()
        .replace(&text, "");

    text.to_string()
}
