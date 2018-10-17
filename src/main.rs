extern crate reqwest;

// use reqwest::StatusCode;

fn main() {
    // long way
    match reqwest::get("https://jsonplaceholder.typicode.com/posts") {
        Ok(mut response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => println!("Response text: {}", text),
                    Err(e) => println!("Could not get text: {}", e),
                }
            } else {
                println!("Response was not 200 OK.")
            }
        }
        Err(e) => println!("Could not make the reqeust. {}", e),
    }

    // short way
    let response_text = reqwest::get("https://jsonplaceholder.typicode.com/posts")
        .expect("Could not make the reqeust")
        .text()
        .expect("Could not get text");
    println!("{}", response_text);
}
