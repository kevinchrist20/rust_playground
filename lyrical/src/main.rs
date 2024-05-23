const BASE_URL: &str = "https://www.azlyrics.com/";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get(BASE_URL).await?;
    println!("Status: {}", response.status());

    let page_html = response.text().await?;
    println!("{}", page_html);

    Ok(())
}


async fn get_lyrics(artist: String, song: String) {
    
}