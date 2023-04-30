use fancy_regex::Regex;
use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path = args().collect::<Vec<_>>()[1..].join("");

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"\{\{\s*(?P<content>.*?)\s*}}").unwrap();
    let re_str = Regex::new(r#"(?P<quot>['"])(?P<content>.*?)\k<quot>"#).unwrap();

    for l in reader.lines() {
        let line = l.unwrap();

        let mut pairs = HashMap::new();
        for c in re.captures_iter(&line) {
            let caps = c.unwrap();

            let text = caps.get(0).unwrap().as_str();
            let content = &caps["content"];
            if let Ok(Some(m)) = re_str.captures(&content) {
                pairs.insert(text, m.name("content").unwrap().as_str().to_string());
            } else {
                let res = meval::eval_str(&content).unwrap();
                pairs.insert(text, res.to_string());
            }
        }

        let mut lm = line.clone();
        for (k, v) in pairs {
            lm = lm.replace(&k, &v);
        }
        println!("{}", lm);
    }
}
