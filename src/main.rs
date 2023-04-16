mod utils {
    pub mod generate_readme;
}
use utils::generate_readme::generate_readme;
use std::io;

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
fn read_line(prompt: &str) -> String {
    println!{"{}", prompt};
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    welcome();
    let mut data = ReadMeData {
        github_name: read_line("Please enter your github username (must be exact)"),
        repo_name: read_line("Please enter repo name (must be exact)"),
        project_name: read_line("Please enter project name"),
        project_description: read_line("Please enter project description"),
        project_bio: read_line("Please enter project bio"),
        project_skills: read_line("Please enter project skills"),
        project_roadmap: read_line("Please enter project roadmap"),
        project_license: read_line("Please enter project license")
    };
    generate_readme(data);

}
