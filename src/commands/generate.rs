use clap::{Command, Arg};
use crate::commands::{hexagonal, saga, clean_code};

pub fn generate_project() {
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
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Type of architecture (hexagonal, saga, clean_code)")
                .action(clap::ArgAction::Set)
                .required(true) // Make this argument required
        )
        .get_matches();

    // Get the values of the parameters or use default values
    let project_name = matches.get_one::<String>("name").unwrap_or(&"my_ts_microservice".to_string()).to_string();
    let version = matches.get_one::<String>("version").unwrap_or(&"1.0".to_string()).to_string();
    let author = matches.get_one::<String>("author").unwrap_or(&"Unknown".to_string()).to_string();
    let about = matches.get_one::<String>("about").unwrap_or(&"A TypeScript microservice".to_string()).to_string();
    let architecture_type = matches.get_one::<String>("type").unwrap_or(&"hexagonal".to_string()).to_string();

    match architecture_type.as_str() {
        "hexagonal" => hexagonal::generate(project_name, version, author, about),
        "saga" => saga::generate(project_name, version, author, about),
        "clean_code" => clean_code::generate(project_name, version, author, about),
        _ => eprintln!("Unsupported architecture type: {}", architecture_type),
    }
}
