use std::env;

use regex::Regex;

use crate::mybible::main as fetch_verse;
mod mybible;
mod interpreter;

use colored::*;
use roxmltree::Document;

pub struct BibleReference {
    pub book: String,
    pub chapter: Vec<u32>,
    pub verse: Vec<u32>,
}

fn style_xml(xml: &str) -> Result<String, roxmltree::Error> {
    let re = Regex::new(r"<S>.*?</S>").unwrap();
    // println!("{}", xml);
    let xml = format!("<verse>{}</verse>", re.replace_all(&xml.replace("<pb/>", ""), ""));
    let doc = Document::parse(&xml)?;
    let mut styled_output = String::new();

    for node in doc.descendants() {
        if node.is_text() {
            if node.parent().unwrap().has_tag_name("i") {
                continue;
            }
            styled_output.push_str(&node.text().unwrap_or("").normal().to_string());
        } else if node.has_tag_name("i") {
            styled_output.push_str(&node.first_child().unwrap().text().unwrap_or("").green().to_string());
        }
    }
    Ok(styled_output)
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let bible_ref: String = args[1].clone();
        let parsed_bible_ref = interpreter::parse(bible_ref.clone());

        match fetch_verse(parsed_bible_ref) {
            Ok(verse) => match style_xml(&verse) {Ok(parsed_verse) => println!("{}", parsed_verse), Err(e) => eprintln!("Error parsing verse XML: {}", e),},
            Err(e) => eprintln!("Error fetching verse: {}", e),
        }

    };
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() > 1 {
//         let bible_ref: String = args[1].clone();
//         if let Some(parsed_bible_ref) = interpreter::parse(bible_ref.clone()) {
//             println!("Book: {}", parsed_bible_ref.book);
//             println!("Chapter: {:?}", parsed_bible_ref.chapter);
//             println!("Verse: {:?}", parsed_bible_ref.verse);
//         }
//     } else {
//         println!("You didn't tell me anything :(")
//     }

// }
