use reqwest::Client;
use std::env;
use std::error::Error;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

// WARN: deprecated
#[allow(dead_code)]
async fn find_valid_a_number(client: &Client, lecture_id: &str) -> Option<u32> {
    println!("Finding the correct 'a' number (a0 to a99)...");
    for a_num in 1..100 {
        let url = format!(
            "https://tqozw4vr7987.edge.naverncp.com/hls/b6yd7gaFWdlIc3gQXQj~-Q__/{}/mp4/{}.mp4/segment-1-v1-a{}.ts",
            lecture_id, lecture_id, a_num
        );
        // only HEAD req
        match client.head(&url).send().await {
            Ok(response) if response.status().is_success() => {
                println!("Found valid 'a' number: a{}", a_num);
                return Some(a_num);
            }
            _ => {
                print!("\rChecking a{}...", a_num);
            }
        }
    }
    None
}

async fn download_stream(client: &Client, lecture_id: &str) -> Result<(), Box<dyn Error>> {
    tokio::fs::create_dir_all("./output").await?;
    let file_path = format!("./output/{}.ts", lecture_id);
    println!("\nOpening output file... {}", file_path);
    let mut output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .await?;

    println!("\nStarting download...");

    let mut seg_number = 1;
    loop {
        let url = format!(
            "https://tqozw4vr7987.edge.naverncp.com/hls/b6yd7gaFWdlIc3gQXQj~-Q__/{}/mp4/{}.mp4/segment-{}-v1-a{}.ts",
            lecture_id,
            lecture_id,
            seg_number,
            1 // TODO: why this number is 1?
        );

        let response = client.get(&url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                // 200 OK: successfully downloaded
                let bytes = response.bytes().await?;
                output_file.write_all(&bytes).await?;
                println!(
                    "Successfully downloaded and appended segment-{}",
                    seg_number
                );
            }
            reqwest::StatusCode::NOT_FOUND => {
                // 404 Not Found: end of video
                println!(
                    "Reached end of video (404 Not Found for segment-{}).",
                    seg_number
                );
                break;
            }
            status => {
                // other error code: consider as error and break
                eprintln!(
                    "Error downloading segment-{}: HTTP Status {}",
                    seg_number, status
                );
                break;
            }
        }
        seg_number += 1;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("test\n{:?}", args);
        eprintln!("This program is best used via Makefile.");
        eprintln!("Usage: make video <lecture_id>");
        eprintln!("   or: make audio <lecture_id>");
        return Ok(());
    }

    let _ = &args[1]; // mode
    let lecture_id = &args[2];

    let client = Client::new();

    // let a_number = match find_valid_a_number(&client, lecture_id).await {
    //     Some(num) => num,
    //     None => {
    //         eprintln!(
    //             "Could not find a valid 'a' number for segment-1. Please check the lecture ID."
    //         );
    //         return Ok(());
    //     }
    // };

    match download_stream(&client, lecture_id).await {
        Ok(_) => {
            println!("\nDownload complete! Video saved as {}", lecture_id);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error downloading video: {}", e);
            Err(e)
        }
    }
}
