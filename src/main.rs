use clap::{Command, Arg};
use std::fs;
use std::path::Path;
use tera::{Context, Tera};

fn main() {
    // Define and parse command-line arguments
    let matches = Command::new("TypeScript Microservice Generator")
        .disable_version_flag(true)
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .value_name("VERSION")
                .help("Version of the project")
                .action(clap::ArgAction::Set)
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Name of the project")
                .action(clap::ArgAction::Set)
        )
        .arg(
            Arg::new("author")
                .short('a')
                .long("author")
                .value_name("AUTHOR")
                .help("Author of the project")
                .action(clap::ArgAction::Set)
        )
        .arg(
            Arg::new("about")
                .short('b')
                .long("about")
                .value_name("ABOUT")
                .help("Description of the project")
                .action(clap::ArgAction::Set)
        )
        .get_matches();

    // Get the values of the parameters or use default values
    let project_name = matches.get_one::<String>("name").unwrap_or(&"my_ts_microservice".to_string()).to_string();
    let version = matches.get_one::<String>("version").unwrap_or(&"1.0".to_string()).to_string();
    let author = matches.get_one::<String>("author").unwrap_or(&"Unknown".to_string()).to_string();
    let about = matches.get_one::<String>("about").unwrap_or(&"A TypeScript microservice".to_string()).to_string();

    let project_path = Path::new(&project_name);

    // Check if the project directory already exists
    if project_path.exists() {
        eprintln!("The directory {} already exists", project_name);
        return;
    }

    // Create the project directory
    fs::create_dir_all(project_path).expect("Failed to create project directory");

    // Initialize Tera with templates
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Error loading templates");

    // Create a context for the templates
    let mut context = Context::new();
    context.insert("project_name", &project_name);
    context.insert("project_version", &version);
    context.insert("project_author", &author);
    context.insert("project_description", &about);

    // Render and write the project files
    let files = vec!["package.json.tera", "tsconfig.json.tera"];
    for file in files {
        let rendered = tera.render(file, &context).expect("Error rendering template");
        let file_name = file.trim_end_matches(".tera");
        let file_path = project_path.join(file_name);
        fs::write(file_path, rendered).expect("Error writing file");
    }

    // Create additional directories
    let dirs = vec!["src", "src/controllers", "src/models", "src/services", "src/config"];
    for dir in dirs {
        fs::create_dir_all(project_path.join(dir)).expect("Error creating directory");
    }

    // Create a basic index.ts file
    let index_content = r#"import express from 'express';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

app.get('/', (req, res) => {
    res.send('Hello, World!');
});

app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
"#;

    fs::write(project_path.join("src/index.ts"), index_content).expect("Error writing index.ts");

    println!("Project {} generated successfully", project_name);
}
