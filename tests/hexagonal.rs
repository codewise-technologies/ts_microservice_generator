use std::fs;
use std::path::Path;
use ts_microservice_generator::commands::hexagonal::generate;

#[test]
fn test_generate_hexagonal() {
    let project_name = "TestProject".to_string();
    let version = "1.0".to_string();
    let author = "Test Author".to_string();
    let about = "Test Description".to_string();

    generate(project_name.clone(), version.clone(), author.clone(), about.clone());

    assert!(Path::new(&project_name).exists());

    assert!(Path::new(&format!("{}/package.json", project_name)).exists());
    assert!(Path::new(&format!("{}/tsconfig.json", project_name)).exists());

    fs::remove_dir_all(&project_name).expect("Failed to remove test project directory");
}
