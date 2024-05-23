use scraper::Selector;

const BASE_URL: &str = "https://www.azlyrics.com";

#[tokio::main]
async fn main() {
    match get_lyrics("imaginedragons", "wrecked").await {
        Ok(()) => println!("Thank you for coming. Until next time!!"),
        Err(err) => eprintln!("An error occurred: {}", err),
    }
}

async fn get_lyrics(artist: &str, song: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{BASE_URL}/lyrics/{artist}/{song}.html");

    let response = reqwest::get(url).await?;
    let html_content = response.text().await?;

    let html_document = scraper::Html::parse_document(&html_content);

    let all_divs = Selector::parse("div")?;
    let comment_selector = Selector::parse("comment")?;
    let br_selector = Selector::parse("br")?;
    let img_selector = Selector::parse("img")?;

    let lyrics_div: Vec<_> = html_document
        .select(&all_divs)
        .filter(|element| {
            let element_data = element.value();
            element_data.attr("class").is_none() && element_data.attr("id").is_none()
        })
        // .chain(html_document.select(&comment_selector))
        // .chain(html_document.select(&br_selector))
        // .chain(html_document.select(&img_selector))
        .collect();

    // Print the filtered divs
    for div in lyrics_div {
        println!("{}", div.inner_html());
    }

    Ok(())
}
