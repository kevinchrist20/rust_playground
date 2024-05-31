use html2text;
use scraper::Selector;

use std::env::{self};
use std::{fs, io};

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
        Ok(()) => println!("Thank you for coming. Until next time!!"),
        Err(err) => eprintln!("An error occurred: {}", err),
    }
}

async fn get_lyrics(artist: &str, song: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{BASE_URL}/lyrics/{artist}/{song}.html");

    println!("BASE_URL: {}", url);
    let response = reqwest::get(url).await?;
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

    println!("{}", lyrics_content);
    write_to_file(lyrics_content, &filename)?;

    Ok(())
}

fn write_to_file(lyrics: String, filename: &str) -> Result<(), io::Error> {
    fs::write(filename, lyrics)
}
