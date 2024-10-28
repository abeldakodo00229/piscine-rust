use chrono::Datelike;
use std::collections::HashMap;

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut  map :  HashMap<String, u32>= HashMap::new();
    for element in data.members(){
        let date=element["commit"]["author"]["date"].as_str().unwrap_or_default().to_string();
        let counter = map.entry(format_week_number(&date)).or_insert(0);
        *counter += 1;
    }
    return map;
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut  map :  HashMap<String, u32>= HashMap::new();
  

    for element in data.members(){
        let author=element["author"]["login"].as_str().unwrap_or_default().to_string();
        let counter = map.entry(author).or_insert(0);
        *counter += 1;
    }
    return map;
}



pub fn format_week_number(date_str: &str) -> String {

    if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(date_str) {
        let week_number = datetime.iso_week().week();
        let year = datetime.year();
        format!("{}-W{}", year, week_number)
    } else {
        String::from("Invalid date format")
    }
}
