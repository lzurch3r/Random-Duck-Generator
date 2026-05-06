use std::fs;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Initialize variables
    let filename = format!("src/favorites.txt");
    let url = format!("https://ducks.now/api/v0/random");

    //Prompt user to view new image or view favorites list
    println!("MAIN MENU\nType a number:");
    println!("1. View random duck image");
    println!("2. View favorites list");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //If view new image
    match input.trim().parse::<u32>() {
        Ok(1) => {
            println!("Looking for ducks...");

            //Send API request
            let response = reqwest::get(&url)
                .await?
                .json::<serde_json::Value>()
                .await?;

            //Get API response
            let api_info = build_api_info(response["detail_url"].to_string(),
                                                response["title"].to_string(),
                                                response["description"].to_string());

            //Display API info
            println!("Duck found!{}", api_info.display());
        }

        //View favorites list
        Ok(2) => {
            //Reading from .txt file
            let favorites = read_file(&filename);
            for duck in favorites {
                println!("{}", duck);
            }
        }

        _ => {
            println!("Invalid option. Please enter 1 or 2.");
        }
    }

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

fn read_file(filename: &String) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    contents.lines().map(|l| l.to_string()).collect()
}

fn save_to_file(filename: String) {
    println!("File {} has been saved!", filename);
}