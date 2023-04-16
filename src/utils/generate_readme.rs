
use std::fs::File;
use std::io::Write;

use crate::ReadMeData;


pub fn generate_readme(data: ReadMeData) -> std::io::Result<()>{
    let mut file = File::create("README.md")?;
   let contents = format!(
    "<a name=\"readme-top\"></a>

  ![GitHub contributors](https://img.shields.io/github/contributors/{}/{}?color=%23454B1B&label=CONTRIBUTORS%20%3C3&style=for-the-badge)
  ![GitHub forks](https://img.shields.io/github/forks/{}/{}?style=for-the-badge)
  ![GitHub Repo stars](https://img.shields.io/github/stars/{}/{}?style=for-the-badge)
  ![GitHub](https://img.shields.io/github/license/{}/{}?style=for-the-badge)
    
    
    
    
  <div align=\"center\">
  <h3 align=\"center\">{}</h3>
  <p align=\"center\">
   {}
  <br />
  <br />
  <a href=\"https://github.com/{}/{}\">View Demo</a>
   ·
        <a href=\"https://github.com/{}/{}/issues\">Report Bug</a>
        ·
        <a href=\"https://github.com/{}/{}/issues\">Request Feature</a>
      </p>
    </div>


  <!-- TABLE OF CONTENT -->
  <details>
      <summary>Table of Contents</summary>
      <ol>
        <li>
          <a href=\"#about-the-project\">About The Project</a>
          <ul>
            <li><a href=\"#built-with\">Built With</a></li>
          </ul>
        </li>
        <li>
          <a href=\"#getting-started\">Getting Started</a>
          <ul>
            <li><a href=\"#prerequisites\">Prerequisites</a></li>
            <li><a href=\"#installation\">Installation</a></li>
          </ul>
        </li>
        <li><a href=\"#usage\">Usage</a></li>
        <li><a href=\"#roadmap\">Roadmap</a></li>
        <li><a href=\"#contributing\">Contributing</a></li>
        <li><a href=\"#license\">License</a></li>
        <li><a href=\"#contact\">Contact</a></li>
      </ol>
    </details>
    
    
  <!-- ABOUT THE PROJECT -->
  ## About The Project
    
    
    
    
  {}
    
    
    
  <p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>
    
    
    
  ### Built With
    
    
    
  {}
    
  <p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>
    
    
    
  <!-- GETTING STARTED -->
  ## Getting Started
    
  If you would like to clone the repo hopefully this will help.
    
  ### Prerequisites
    
  {}
    
  ### Installation
    
  1. Clone the repo
        ```sh
        git clone https://github.com/{}/{}.git
        ```
  2. Install NPM packages in both client and server folders
        ```sh
        npm install
        ```
  3. Create a .env file in the server folder and add your own MongoDB URI
        ```sh
        MONGODB_URI=YOUR_OWN_MONGODB_URI
        ```
  4. Run the start commands in both client and server folders (commands are in the package.json files).
        ```sh
        npm start / npm run dev
        ```
  5. Enjoy! made with <3 by {}
  <p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>
    
    
    
  <!-- USAGE EXAMPLES -->
  ## Usage
    
  {}
  <p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>
    
    
    
   <!-- ROADMAP -->
   ## Roadmap
    
  - [] {}
    
    
  See the [open issues](https://github.com/{}/{}/issues) for a full list of future proposed features (and known issues).
    
   <p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>
    
    
    
   <!-- CONTRIBUTING -->
   ## Contributing
    
  If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag \"enhancement\".
  Don't forget to give the project a star! Thanks again!
    
  1. Fork the Project
  2. Create your Feature Branch (\\`git checkout -b feature/AmazingFeature\\`)
  3. Commit your Changes (\\`git commit -m 'Add some AmazingFeature'\\`)
  4. Push to the Branch (\\`git push origin feature/AmazingFeature\\`)
  5. Open a Pull Request
    
   <p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>
    
    
    
   <!-- LICENSE -->
   ## License
    
  {}
    
   <p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>
    
    
    
   <!-- CONTACT -->
   ## Contact
    
  Just message me on github mang <3 {}
    
  Project Link: [https://github.com/{}/{}](https://github.com/{}/{})
    
   <p align=\"right\">(<a href=\"#readme-top\">back to top</a>)</p>
   ", 
   data.github_name, 
   data.repo_name, 
   data.github_name, 
   data.repo_name, 
   data.github_name, 
   data.repo_name, 
   data.github_name, 
   data.repo_name, 
   data.project_name, 
   data.project_bio, 
   data.github_name, 
   data.repo_name, 
   data.github_name, 
   data.repo_name, 
   data.github_name, 
   data.repo_name, 
   data.project_description, 
   data.project_skills,
   data.project_prerequisites,
   data.github_name, 
   data.repo_name, 
   data.github_name,
  data.project_usage,
  data.project_roadmap,
  data.github_name, 
  data.repo_name, 
  data.project_license,
  data.github_name, 
  data.github_name, 
  data.repo_name, 
  data.github_name, 
  data.repo_name  );

    file.write_all(contents.as_bytes())?;
    Ok(())

}