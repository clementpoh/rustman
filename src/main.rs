use std::{env, error::Error};

use axum::{http::header::HeaderMap, response::Html, routing::get, Router};
use tokio::{fs::File, io::AsyncReadExt};

const INDEX: &str = "assets/index.html";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let index = get_page_content().await?;

    eprintln!("Working directory: {:?}", env::current_dir().unwrap());

    let app = Router::new().route("/", get(|headers| root(headers, index)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_page_content() -> Result<String, Box<dyn Error>> {
    let mut file = File::open(INDEX).await?;
    let mut index = vec![];
    file.read_to_end(&mut index).await?;

    // let index = fs::read_to_string(INDEX).expect("Could not read index.html");
    Ok(String::from_utf8(index)?)
}

async fn root(headers: HeaderMap, index: String) -> Html<String> {
    eprintln!("{:?}", headers);
    // format!("{:#?}", headers)
    Html(index)
}
