use select::document::Document;
use select::predicate::{Name};
use std::env;

fn main() {
    // Def
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    // URL
    let mut url = String::from("https://en.wikipedia.org/wiki/");
    url.push_str(&input);

    // APU call
    let resp = reqwest::blocking::get(&url).expect("Api call failed.");
    let resp = resp.text().expect("Failed while extracting body.");

    // Parse
    let document = Document::from(&resp[..]);
    for node in document.select(Name("p")).take(2).skip(1) {
        let tag = node.next().unwrap().text();
        println!("{}", tag);
    }
}
