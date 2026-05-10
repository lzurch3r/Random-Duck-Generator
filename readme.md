# Random Duck Generator

A terminal-based program where the user can choose to get a duck image url from an API or view a list of urls for their favorite duck images.

## Instructions for Build and Use

Steps to build and/or run the software:

1. Download Rust version 1.95.0
2. Open Windows PowerShell from the Start Menu
3. Change the directory to the folder containing code (ex. "cd ...random_duck_generator")
4. Type "cargo run" into terminal and press Enter to run the software in the terminal

Instructions for using the software:

1. From the main menu, press 1 to generate a url to a random duck image or press 2 to view a list of your saved favorites.
2. If 1 was pressed, a url will be generated, and you can copy and paste the url into your preferred browser to view the duck image.
3. You will be prompted to save the url as a favorite. Press "y" to save the duck or "n" to end the program.
4. If 2 was pressed in the main menu, a list of saved favorites will show up in the terminal and you can copy and paste any of the urls into your preferred browser. If there are no favorites saved, you will be shown a message and the program will end.
5. Note: for most of the final tasks in the program, the program will end, so you will need to type "cargo run" into the terminal again and press Enter to run the program once more. Enjoy!

## Development Environment

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Visual Studio Code version 1.119.0
* Rust version 1.95.0 (59807616e 2026-04-14)
* rust-analyzer extension on Visual Studio Code

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [Rust Programming Language](https://doc.rust-lang.org/stable/book/)
* [Rust for Data](https://rustfordata.com/chapter_3.html)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] Add feature to allow the program to loop back after failed user input
* [ ] Add feature to allow the program to loop to the main menu after completing any final task
