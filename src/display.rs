use std::collections::BTreeMap;

pub fn display_result(data: &BTreeMap<String, Vec<String>>, filter_keys: Option<&[&str]>) {
    for (key, values) in data {
        if filter_keys.map_or(true, |keys| keys.contains(&key.as_str())) {
            if values.len() > 1 {
                println!("\x1b[32m{}\x1b[0m:", key);
                for value in values {
                    println!("\t\x1b[34m{}\x1b[0m", value);
                }
            } else if let Some(value) = values.get(0) {
                println!("\x1b[32m{}\x1b[0m: \x1b[34m{}\x1b[0m", key, value);
            }
        }
    }
}