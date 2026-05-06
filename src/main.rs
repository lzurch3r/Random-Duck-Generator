use std::fs;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Initialize variables
    let filename = format!("src/ducks.txt");
    let url = format!("https://ducks.now/api/v0/random");

    //Prompt user to view new image or favorites list
    println!("View new image or view favorites list:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //Send API request
    let response = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    //Get API response
    let api_info = build_api_info(response["detail_url"].to_string(),
                                           response["title"].to_string(),
                                           response["description"].to_string());

    //Display API info
    println!("Duck found!{}", api_info.display());
    
    //Reading from .txt file
    let contents: String = read_file(&filename);
    println!("{contents}");

    //Saving to .txt file
    save_to_file(String::from("duck_file"));

    Ok(())
}
struct ApiInfo {
    detail_url: String,
    title: String,
    description: String,
}

impl ApiInfo {
    fn display(&self) -> String {
        format!("\nImage URL (copy and paste into browser): {}\nTitle: {}\nDescription: {}",
            self.detail_url, self.title, self.description)
    }
}

fn build_api_info(detail_url: String, title: String, description: String) -> ApiInfo {
    ApiInfo {
        detail_url,
        title,
        description,
    }
}

fn read_file(filename: &String) -> String {
    //println!("Reading file \"{}\"...", filename);
    fs::read_to_string(filename).expect("Can't read file")
    //println!("Got file {}!", filename);
    //println!("File contents: {}", contents);
}

fn save_to_file(filename: String) {
    println!("File {} has been saved!", filename);
}