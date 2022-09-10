use std::collections::HashMap;

use chrono::{NaiveDate, Datelike};

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for commit in data.clone().members() {
        for d in commit.entries() {
            if d.0 == "commit" {
                for a in (d.1).clone().entries() {
                    if a.0 == "author" {
                        for date in (a.1).clone().entries() {
                            if date.0 == "date" {
                                let week = (date.1).to_string();
                                let ywd = week.split('-').collect::<Vec<&str>>();
                                let day = ywd[2].split_at(2);
                                let mut str  = String::new();
                                let d = NaiveDate::from_ymd(ywd[0].to_string().parse().unwrap(), ywd[1].to_string().parse().unwrap(), day.0.to_string().parse().unwrap()).iso_week();
                                str.push_str(&d.year().to_string());
                                str.push_str("-W");
                                str.push_str(&d.week().to_string());
                                if !map.contains_key(&str) {
                                    map.insert(str, 1);
                                } else {
                                    *map.get_mut(&str).unwrap() += 1;
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", map);
    map
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    // let data1 = data.clone();
    let mut map = HashMap::new();
    for commit in data.clone().members() {
        for d in commit.entries() {
            //   println!("{:?}", d.to_owned());
            //   println!("==================");
            if d.0 == "author" {
                // println!("{:?}||||||||||", (d.1));
                for a in (d.1).clone().entries() {
                    if a.0 == "login" {
                        let name = (a.1).to_string();
                        if !map.contains_key(&name) {
                            map.insert(name, 1);
                        } else {
                            *map.get_mut(&name).unwrap() += 1;
                        }
                    }
                }
                // println!("Author!");
            }
        }
    }
    println!("{:?}", map);
    map
}
