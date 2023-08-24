mod utils {
    pub mod generate_readme;
}
use utils::generate_readme::generate_readme;
use std::io;
use colored::*;

pub struct ReadMeData {
    github_name: String,
    repo_name: String,
    project_name: String,
    project_description: String,
    project_bio: String,
    project_skills: String,
    project_prerequisites: String,
    project_usage: String,
    project_roadmap: String,
    project_license: String,


}

fn prompt_user(prompt: &str) -> String {
    println!{"{}", prompt};
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("\n\n\n\n\n\n");
    let welcome  = "Welcome to imjord readme generator made in rust";
    let welcome_uppercase = welcome.to_uppercase();
    println!("{}", welcome_uppercase.green().bold().italic());
    println!("{}", "-----------------------------------------------".green());
    let data = ReadMeData {
        github_name: prompt_user("Please enter your github username (must be exact)"),
        repo_name: prompt_user("Please enter repo name (must be exact)"),
        project_name: prompt_user("Please enter project name"),
        project_description: prompt_user("Please enter project description"),
        project_bio: prompt_user("Please enter project bio"),
        project_skills: prompt_user("Please enter project skills"),
        project_prerequisites: prompt_user("Please enter project prerequisites"),
        project_usage: prompt_user("Please enter project usage / links to demo"),
        project_roadmap: prompt_user("Please enter project roadmap"),
        project_license: prompt_user("Please enter project license")
    };
    match generate_readme(data) {
        Ok(()) => println!("readme generated successfully"),
        Err(e) => println!("error trying to create the readme: {}", e),
    }

}
