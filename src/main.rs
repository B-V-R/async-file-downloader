use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use futures::stream::StreamExt;

fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<String> = read_lines("src/urls.txt")?;
    let fetches = futures::stream::iter(
        paths.into_iter().map(|path| {
            async move {
                match reqwest::get(&path).await {
                    Ok(resp) => {
                        match resp.text().await {
                            Ok(text) => {
                                println!("RESPONSE: {} bytes from {}", text.len(), path);
                            }
                            Err(_) => println!("ERROR reading {}", path),
                        }
                    }
                    Err(_) => println!("ERROR downloading {}", path),
                }
            }
        })
    ).buffer_unordered(15).collect::<Vec<()>>();
    fetches.await;
    Ok(())
}