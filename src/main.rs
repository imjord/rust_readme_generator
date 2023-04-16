mod utils {
    pub mod generate_readme;
}

use utils::generate_readme::generate_readme;
use std::fs;
use std::fs::File;
use std::io;
use dialoguer::{theme::ColorfulTheme, Select};


pub struct ReadMeData {
    github_name: String,
    repo_name: String
}

fn welcome(){
    let welcome  = "Welcome to imjord readme generator made in rust";
    let welcome_uppercase = welcome.to_uppercase();
    println!("{}", welcome_uppercase);
    println!("----------------------------------------");

}   

fn main() {
    welcome();
    let mut data = ReadMeData {
        github_name: String::new(),
        repo_name: String::new()
    };

    println!("Please enter your github username / must be exact");
    io::stdin().read_line(&mut data.github_name);
    println!("Please enter repo name / must be exact");
    io::stdin().read_line(&mut data.repo_name);

    println!("{:?}", data.github_name.trim());
    generate_readme(data);
    // let selections = &[
    //     "Ice Cream",
    //     "Vanilla Cupcake",
    //     "Chocolate Muffin",
    //     "A Pile of sweet, sweet mustard",
    // ];

    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Pick your flavor")
    //     .default(0)
    //     .items(&selections[..])
    //     .interact()
    //     .unwrap();

    // println!("Enjoy your {}!", selections[selection]);

    
}
