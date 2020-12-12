use regex::Regex;
use chrono::{DateTime};

pub fn get_single_bracket_data(line: &'_ str) -> Result<&'_ str, &str> {
    lazy_static! {
        static ref BRACKET_REGEX: Regex = Regex::new(r"\((.*?)\)").unwrap();
    }
    let caps_res = BRACKET_REGEX.captures(line);
    if caps_res.is_none() {
        return Err("Incomplete data");
    }
    let caps = caps_res.unwrap();
    Ok(caps.get(1).map_or("", |m| m.as_str()))
}

pub fn get_double_bracket_data(line: &'_ str) -> Result<Vec<&'_ str>, &str> {
    lazy_static! {
        static ref BRACKET_REGEX: Regex = Regex::new(r"\((.*?)\)\((.*?)\)").unwrap();
    }
    let caps_res = BRACKET_REGEX.captures(line);
    if caps_res.is_none() {
        return Err("Incomplete data");
    }
    let caps = caps_res.unwrap();

    let str_caps: Vec<&str> = caps
    .iter()
    .skip(1)
    .filter(|m| m.is_some())
    .map(|m| m.unwrap().as_str())
    .collect();

    if str_caps.len() < 2 {
        return Err("Incomplete data");
    }

    Ok(str_caps)
}

pub fn parse_numeric(usage: &str) -> String {
    lazy_static! {
        static ref NON_NUMERIC: Regex = Regex::new(r"[^0-9]").unwrap();
    }

    format!("{}", NON_NUMERIC.replace_all(usage, ""))
}

pub fn divide_by_thousand(gas_volume_str: &str) -> f64 {
    let volume_float = gas_volume_str.parse::<f64>().unwrap_or_default();
    volume_float / 1000.0
}

pub fn parse_timestamp(timestamp: &str) -> i64 {
    let mut parse_time = format!("20{}-{}-{}T{}:{}:{}", 
    &timestamp[0..2], &timestamp[2..4], &timestamp[4..6], &timestamp[6..8], &timestamp[8..10], &timestamp[10..12]);
    
    if &timestamp[12..13] == "W" {
        parse_time += "+01:00";
    } else {
        parse_time += "+02:00";
    }

    let rfc3339_opt = DateTime::parse_from_rfc3339(&parse_time);
    if rfc3339_opt.is_err() {
        return 0;
    }

    rfc3339_opt.unwrap().timestamp()
}
