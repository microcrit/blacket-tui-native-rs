use std::collections::HashMap;

pub fn parse_cookie(cookie: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for part in cookie.split("; ") {
        let mut split = part.split("=");
        if split.clone().count() != 2 {
            continue;
        }
        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap().to_string();  
        map.insert(key, value);
    }
    map
}

pub fn build_cookie(map: &HashMap<String, String>) -> String {
    let mut cookie = String::new();
    for (key, value) in map {
        cookie.push_str(&format!("{}={}; ", key, value));
    }
    cookie = cookie.trim_end_matches("; ").to_string();
    cookie
}