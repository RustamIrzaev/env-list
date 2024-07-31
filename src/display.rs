use std::collections::BTreeMap;
use colored::Colorize;

pub fn display_result(data: &BTreeMap<String, Vec<String>>, filter_keys: Option<&[&str]>) {
    for (key, values) in data {
        if filter_keys.map_or(true, |keys| keys.contains(&key.as_str())) {
            if values.len() > 1 {
                println!("{}: ", key.green());

                for value in values {
                    println!("\t{}", value.blue());
                }
            } else if let Some(value) = values.get(0) {
                println!("{}: {}", key.green(), value.blue());
            }
        }
    }
}