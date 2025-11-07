use std::{fs::File, io::Read, str::FromStr};

use chrono::{DateTime, Datelike, Days, Local, Weekday};
use regex::Regex;



pub fn read_token_file(path: &str) -> String {
    let mut f = File::open(path).unwrap();
    let mut token = "".to_string();
    let _ = f.read_to_string(&mut token);
    token
}

pub fn parse_id(id: &str) -> Option<String> {
    if let Some(no_pref) = id.strip_prefix("<@") {
        if let Some(no_suf) = no_pref.strip_suffix(">") {
            return Some(no_suf.to_string());
        }
    }
    return None;
}

#[derive(Debug)]
pub struct DateError;

/// matches dd/mm, d/m, y/m/d
fn match_date(string: &str, year: i32) -> Option<String> {
    let dm = Regex::new(r"^(?<day>\d{1,2})(-)(?<month>\d{1,2})$").unwrap();
    if let Some(caps) = dm.captures(string) {
        let month = if caps["month"].len() == 1 {format!("0{}", &caps["month"])} else {caps["month"].to_string()};
        let day = if caps["day"].len() == 1 {format!("0{}", &caps["day"])} else {caps["day"].to_string()};

        return Some(format!("{}-{}-{}", year, month, day));
    }

    let ymd = Regex::new(r"^(?<year>\d{2,4})-(?<month>\d{1,2})-(?<day>\d{1,2})$").unwrap();
    if let Some(caps) = ymd.captures(string) { // this will break in the year 3000, issue for later
        let year = if caps["year"].len() == 2 {format!("20{}", &caps["year"])} else {caps["year"].to_string()};
        let month = if caps["month"].len() == 1 {format!("0{}", &caps["month"])} else {caps["month"].to_string()};
        let day = if caps["day"].len() == 1 {format!("0{}", &caps["day"])} else {caps["day"].to_string()};
        return Some(format!("{}-{}-{}", year, month, day));
    }
    None
}

fn next_day(day: &str, current_date: DateTime<Local>) -> Option<String> {

    let current_day = current_date.weekday();
    if let Ok(request_day) = Weekday::from_str(day) {
        let offset = request_day.days_since(current_day);
        if let Some(request_date) = current_date.checked_add_days(Days::new(offset as u64)) {
            return Some(request_date.format("%Y-%m-%d").to_string());
        }
    }

    None
}

pub fn process_day(string: &str) -> Result<String, DateError> {

    let now = Local::now();
    let lower_string = string.to_lowercase();

    // string date
    if let Some(res) = match_date(string, now.year()) {
        return Ok(res);
    }

    // string day
    if let Some(res) = next_day(&lower_string, now) {
        return Ok(res);
    }

    Err(DateError)
}


#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn date_match_test() {
        assert_eq!(match_date("15-07", 2025).unwrap(), "2025-07-15".to_string());
        assert_eq!(match_date("15-7", 2025).unwrap(), "2025-07-15".to_string());
        assert_eq!(match_date("05-17", 2025).unwrap(), "2025-17-05".to_string());
        assert_eq!(match_date("5-17", 2025).unwrap(), "2025-17-05".to_string());
        assert_eq!(match_date("2025-11-06", 2025).unwrap(), "2025-11-06".to_string());
        assert_eq!(match_date("25-11-06", 2025).unwrap(), "2025-11-06".to_string());
    }

    #[test]
    fn next_day_test() {
        let local = Local;
        let now = local.with_ymd_and_hms(2025, 10, 20, 1, 1, 1).unwrap();

        assert_eq!(next_day("monday", now), Some("2025-10-20".to_string()));
        assert_eq!(next_day("wednesday", now), Some("2025-10-22".to_string()));
        assert_eq!(next_day("wed", now), Some("2025-10-22".to_string()));
        assert_eq!(next_day("asufoag", now), None);
    }
}