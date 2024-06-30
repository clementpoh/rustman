use std::{env, error::Error};

use axum::{http::header::HeaderMap, response::Html, routing::get, Router};
use tokio::{fs::File, io::AsyncReadExt};

const INDEX_FILE: &str = "assets/index.rs.html";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let content = get_page_content().await?;
    let app = Router::new().route("/", get(|headers| root(headers, content)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    eprintln!(
        "Working directory: {:?}",
        env::current_dir().unwrap_or("Could not get working directory".into())
    );

    Ok(())
}

async fn get_page_content() -> Result<String, Box<dyn Error>> {
    let mut file = File::open(INDEX_FILE)
        .await
        .expect(&format!("Could not open `{}`", INDEX_FILE));

    let mut content = vec![];
    file.read_to_end(&mut content)
        .await
        .expect(&format!("Could not read `{}`", INDEX_FILE));

    Ok(String::from_utf8(content).expect(&format!("Could not convert {}", INDEX_FILE)))
}

async fn root(headers: HeaderMap, content: String) -> Html<String> {
    eprintln!("{:?}", headers);
    // format!("{:#?}", headers)
    Html(content)
}
