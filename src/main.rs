#![allow(non_snake_case)]
use std::{collections::HashMap};

pub enum Hsval{
    Hs(HashMap<String, Hsval>),
    String(String),
    Int(i16),
}

fn capitlize(mut name: String, count: usize) -> String {
    let mut v: Vec<char> = name.chars().collect();
    v[count] = v[count].to_uppercase().next().unwrap();
    name = v.into_iter().collect();
    name
}

fn formatHashMap(hm: HashMap<String, Hsval>) -> String {
    let mut str = String::new();
    for (key, value) in hm {
        match value {
            Hsval::Hs(hm) => {
                str.push_str(&format!("{}-", key));
                str.push_str(&formatHashMap(hm));
                str.push('\n');
            }
            Hsval::String(s) => {
                if s != "" {
                    str.push_str(&format!("{}:{};", key, s));
                }
            }
            Hsval::Int(i) => {
                str.push_str(&format!("{}:{};", key, i));
            }
        }
    }
    str
}
fn saveHashMap(hm: HashMap<String, Hsval>) {
    let mut file = std::fs::File::create("db.nobl").unwrap();
    let str = formatHashMap(hm);
    {
        use std::io::Write;
        file.write_all(str.as_bytes()).unwrap();
    }
}

fn loadHashMap() -> HashMap<String, Hsval> {
    let mut animes: HashMap<String, Hsval> = HashMap::new();
    let mut file = std::fs::File::open("db.nobl").unwrap();
    let mut str = String::new();
    {
        use std::io::Read;
        file.read_to_string(&mut str).unwrap();
    }
    let str = str.split("\n");
    for s in str { // individual anime
        let mut t = s.split("-"); // name - data
        let name = t.next().unwrap().to_string();
        
        let data = match t.next() {
            Some(s) => s.split(';'),
            None => continue,
        };

        for x in data {
            let mut arg = x.split(":"); // key : value
            let key = arg.next().unwrap();
            let value = match arg.next() {
                Some(s) => s,
                None => "",
            };
            match animes.get_mut(&name) {
                Some(Hsval::Hs(hm)) => {
                    if key != "\r" {
                        hm.insert(key.to_string(), Hsval::String(value.to_string()));
                    }
                }
                None => {
                    let mut hm: HashMap<String, Hsval> = HashMap::new();
                    hm.insert(key.to_string(), Hsval::String(value.to_string()));
                    animes.insert(name.clone(), Hsval::Hs(hm));
                }
                _ => {}
            }
        }
    }
    animes
}

fn addAnime() {
    let mut animes = loadHashMap();
    let mut animeName = String::new();
    let mut tempAnimeSeason = String::new();
    let mut tempAnimeEp = String::new();
    let mut status = String::new();
    
    { // name
        println!("What anime are you think about?");
        std::io::stdin().read_line(&mut animeName).unwrap();
        animeName = animeName.trim().to_string();
        let mut count = 0;
        for x in animeName.clone().chars() {
            if x == '-' {
                println!("Invalid character '-'");
                return;
            }
            else if x == ' ' {
                animeName = capitlize(animeName, count + 1);
            }
            else if count == 0 {
                animeName = capitlize(animeName, count);
            }
            count += 1;
        }
    }

    if animes.contains_key(&animeName) {
        println!("Anime already exists!");
        return;
    }

    { // status
        println!("What is the status of the anime? (Watching, Completed, On Hold, Dropped, Plan to Watch)");
        std::io::stdin().read_line(&mut status).unwrap();
        status = status.trim().to_lowercase();
        status = match status.as_str() {
            "watching" => "Watching".to_string(),
            "completed" | "complete" => "Completed".to_string(),
            "on hold" => "On Hold".to_string(),
            "dropped" => "Dropped".to_string(),
            "plan to watch" | "planned" | "plan"=> "Plan to Watch".to_string(),
            _ => panic!("That's not a status!"),
        };
    }

    let doStatus: bool = match status.as_str() {
        "watching" => true,
        "completed" | "complete" => false,
        "on hold" => true,
        "dropped" => true,
        "plan to watch" | "planned" | "plan" => false,
        _ => false,
    };

    let mut animeSeason: i16 = -1;
    let mut animeEp: i16 = -1;

    if doStatus {
        { // season
            println!("What season are you on?");
            std::io::stdin().read_line(&mut tempAnimeSeason).unwrap();
            tempAnimeSeason = tempAnimeSeason.trim().to_string();
            animeSeason = match tempAnimeSeason.parse::<i16>() {
                Ok(num) => num,
                Err(_) => panic!("That's not a number!"),
            };
        }

        { // episode
            println!("How many episodes have you watched?");
            std::io::stdin().read_line(&mut tempAnimeEp).unwrap();
            tempAnimeEp = tempAnimeEp.trim().to_string();
            animeEp = match tempAnimeEp.parse::<i16>() {
                Ok(num) => num,
                Err(_) => panic!("That's not a number!"),
            };
        }
    }

    animes.insert(animeName.clone(), Hsval::Hs(HashMap::new()));

    match animes.get_mut(&animeName) {
        Some(Hsval::Hs(anime)) => {
            anime.insert("Status".to_string(), Hsval::String(status));
            if doStatus {
                anime.insert("Season".to_string(), Hsval::Int(animeSeason));
                anime.insert("Episodes".to_string(), Hsval::Int(animeEp));
            }
        },
        _ => panic!("Something went wrong!"),
    }
    saveHashMap(animes);
}

fn removeAnime() {
    let mut animes = loadHashMap();
    let mut animeName = String::new();
    println!("What anime do you want to remove?");
    std::io::stdin().read_line(&mut animeName).unwrap();
    animeName = animeName.trim().to_string();
    let mut count = 0;
    for x in animeName.clone().chars() {
        if x == ' ' {
            animeName = capitlize(animeName, count + 1);
        }
        else if count == 0 {
            animeName = capitlize(animeName, count);
        }
        count += 1;
    }

    match animes.remove(&animeName.trim().to_string()) {
        Some(_) => println!("Removed {} from the database.", animeName),
        None => println!("{} is not in the database.", animeName),
    }
    saveHashMap(animes);
}

fn main() {
    println!("Welcome to the anime database!");
    loop {
        println!("What would you like to do?");
        println!("1. Add an anime");
        println!("2. Remove an anime");
        println!("3. Exit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(1) => addAnime(),
            Ok(2) => removeAnime(),
            Ok(3) => break,
            _ => println!("That's not a valid option!"),
        }
    }
}
