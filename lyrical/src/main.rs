use html2text;
use scraper::Selector;

use std::env::{self};
use std::{fs, io};

use regex::Regex;

const BASE_URL: &str = "https://www.azlyrics.com";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Argument must consists of artist and song title.");
        return;
    }

    let artist = &args[1];
    let song_title = &args[2];

    match get_lyrics(&artist, &song_title).await {
        Ok(()) => println!("Song lyrics saved to file successfully! 🎉"),
        Err(err) => eprintln!("An error occurred: {}", err),
    }
}

async fn get_lyrics(artist: &str, song: &str) -> Result<(), Box<dyn std::error::Error>> {
    let artist = normalize_url_str(artist)?;
    let song = normalize_url_str(song)?;
    
    let url = format!("{BASE_URL}/lyrics/{artist}/{song}.html");

    println!("Fetching lyrics from: {}", url);
    let response = reqwest::get(url).await?;
    if response.status() != 200 {
        return Err("Failed to get lyrics. Try again!".into());
    }

    let html_content = response.text().await?;
    let html_document = scraper::Html::parse_document(&html_content);

    let all_divs = Selector::parse("div")?;

    let lyrics_div: Vec<_> = html_document
        .select(&all_divs)
        .filter(|element| {
            let element_data = element.value();
            element_data.attr("class").is_none() && element_data.attr("id").is_none()
        })
        .collect();

    let filename = format!("{artist}_{song}.txt");
    let mut lyrics_content = String::from("");

    for div in lyrics_div {
        let html = div.inner_html();
        let text = html2text::from_read(html.as_bytes(), 80);

        lyrics_content.push_str(&text);
    }

    write_to_file(lyrics_content, &filename)?;

    Ok(())
}

fn write_to_file(lyrics: String, filename: &str) -> Result<(), io::Error> {
    fs::write(filename, lyrics)
}

fn normalize_url_str(path_str: &str) -> Result<String, regex::Error> {
    let lowercase = path_str.to_lowercase();

    // Remove special characters
    let re = Regex::new(r"[^a-z0-9\s]")?;
    let cleaned = re.replace_all(&lowercase, "");

    // Remove spaces
    let re_spaces = Regex::new(r"\s+")?;
    let no_spaces = re_spaces.replace_all(&cleaned, "");

    Ok(no_spaces.to_string())
}
