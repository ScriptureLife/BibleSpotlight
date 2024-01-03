use rusqlite::{params, Connection, Result};

use crate::BibleReference;


pub fn main(bible_reference: Option<BibleReference>) -> Result<String, rusqlite::Error> {
    let conn = Connection::open("bible-versions/KJV+.SQLite3")?;

    let fetch_book = format!("SELECT book_number FROM books WHERE long_name LIKE \"{}\"", bible_reference.as_ref().unwrap().book.trim_end());
    // println!("{:?}", &fetch_book);
    let book_number: u32 = conn.query_row(&fetch_book, params![], |row| row.get(0))?;
   
    let fetch_verse: String;


    if bible_reference.as_ref().unwrap().verse.last().is_none() {
        fetch_verse = format!("SELECT text FROM verses WHERE book_number = {} AND chapter = {}", book_number, bible_reference.as_ref().unwrap().chapter[0]);
    } else {
        fetch_verse = format!("SELECT text FROM verses WHERE book_number = {} AND chapter = {} AND verse BETWEEN {} AND {}", book_number, bible_reference.as_ref().unwrap().chapter[0], bible_reference.as_ref().unwrap().verse[0], bible_reference.as_ref().unwrap().verse.last().unwrap());
    }

    // println!("{:?}", &fetch_verse);

    let mut stmt = conn.prepare(&fetch_verse)?;
    let verse_iter = stmt.query_map(params![], |row| {Ok(row.get::<_, String>(0)?)
    })?;

    let mut verses = String::from("");
    for verse in verse_iter {
        if verses == "" {
            verses = format!{"{}", verse.unwrap().to_string()};
            continue;
        }
        verses = format!{"{} {}", verses, verse.unwrap().to_string()};
    }



    return Ok(verses)

}