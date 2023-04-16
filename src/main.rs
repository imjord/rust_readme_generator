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
    repo_name: String,
    project_name: String,
    project_description: String,
    project_bio: String,
    project_skills: String,
    project_roadmap: String,
    project_license: String,


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
        repo_name: String::new(),
        project_name: String::new(),
        project_description: String::new(),
        project_bio: String::new(),
        project_skills: String::new(),
        project_roadmap: String::new(),
        project_license: String::new(),


    };

    println!("Please enter your github username / must be exact");
    io::stdin().read_line(&mut data.github_name);
    println!("Please enter repo name / must be exact");
    io::stdin().read_line(&mut data.repo_name);
    println!("Please enter project name / must be exact");
    io::stdin().read_line(&mut data.project_name);
    println!("Please enter project description / must be exact");
    io::stdin().read_line(&mut data.project_description);
    println!("Please enter project bio / must be exact");
    io::stdin().read_line(&mut data.project_bio);
    println!("Please enter project skills / must be exact");
    io::stdin().read_line(&mut data.project_skills);
    println!("Please enter project roadmap / must be exact");
    io::stdin().read_line(&mut data.project_roadmap);
    println!("Please enter project license / must be exact");
    io::stdin().read_line(&mut data.project_license);

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
