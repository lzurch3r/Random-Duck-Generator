use std::fs;
use std::io;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //Initialize variables
    let filename = format!("src/favorites.txt");
    let url = format!("https://ducks.now/api/v0/random");

    //Create a favorites document if none exists
    if !Path::new(&filename).exists() {
        fs::write(&filename, "").expect("Failed to create favorites.txt");
    }

    //Prompt user to view new image or view favorites list
    println!("MAIN MENU\nType a number and press Enter:");
    println!("1. View random duck image");
    println!("2. View favorites list");

    let mut input = String::new();

    while input.trim() != "1" || input.trim() != "2" {
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    }

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
            println!("Duck found!\n{}", api_info.display());

            //Prompt user to save duck as favorite
            println!("Would you like to add this precious duck to your favorites? (y)es or (n)o");
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim();

            //Match case to save or not
            match input {
                //If (y)es, save to .txt file
                "y" => {
                    //Read from .txt file into vector
                    let mut favorites = read_file(&filename);

                    //Push new data into vector
                    favorites.push(api_info.save_to_file());

                    //Saving to .txt file
                    save_to_file(favorites, filename.clone());
                }

                //If (n)o, end program
                "n" => {
                    println!("Thanks for watching the ducks!");
                }

                //Default case, end program
                _ => {
                    println!("Invalid option. Please enter y or n.");
                }
            }
        }

        //View favorites list
        Ok(2) => {
            //Reading from .txt file and end program
            let favorites = read_file(&filename);
            
            //Check if favorites file is empty
            if favorites.is_empty() {
                println!("No ducks have been claimed as your favorites (sad quack).");
            }
            else {
                println!("\nHere are your favorite ducks!");
                for duck in favorites {
                    println!("\n{}", duck);
                }
        }
            
        }

        _ => {
            println!("Invalid option. Please enter 1 or 2.");
        }
    }

    //Add whitespace to terminal
    println!("");
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
    fn save_to_file(&self) -> String {
        format!("Image URL (copy and paste into browser): {} Title: {} Description: {}",
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
    let contents = fs::read_to_string(&filename).expect("Failed to read file");
    contents.lines().map(|l| l.to_string()).collect()
}

fn save_to_file(favorites: Vec<String>, filename: String) {
    let contents = favorites.join("\n");
    fs::write(&filename, contents.trim()).expect("Failed to write file");
    println!("You saved a duck (to your favorites)!");
}