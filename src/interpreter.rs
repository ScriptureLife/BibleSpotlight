use regex::Regex;

use crate::BibleReference;


pub fn parse(bible_ref: String) -> Option<BibleReference> { 

    let rgx = Regex::new(r"(?P<book>\D+)(?P<chapter>\d+)(-(?P<end_chapter>\d+))?(:((?P<verse>\d+)(-(?P<end_verse>\d+))?))?").unwrap();
    if let Some(caps) = rgx.captures(&bible_ref) {
        let book = caps.name("book")?.as_str().to_string();
        let chapter = caps.name("chapter")?.as_str().parse().ok()?;
        let end_chapter = caps.name("end_chapter").map(|m| m.as_str().parse().ok()).flatten().unwrap_or(chapter);
        let verse = match caps.name("verse") {
            Some (m) => (m.as_str().parse::<u32>().ok()?..=caps.name("end_verse").map(|m| m.as_str().parse::<u32>().ok()).flatten().unwrap_or(m.as_str().parse().ok()?)).collect(),
            None => Vec::new(),
        };
        return Some(BibleReference { book, chapter: (chapter..=end_chapter).collect(), verse });
    } else {
        return None;
    }


}