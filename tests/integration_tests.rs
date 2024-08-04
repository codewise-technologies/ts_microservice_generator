use std::process::Command;
use std::fs;
use std::path::Path;

#[test]
fn test_generate_project() {
    // Define the project parameters
    let project_name = "TestProject";
    let project_path = Path::new(project_name);

    // Remove any existing project directory from previous test runs
    if project_path.exists() {
        fs::remove_dir_all(project_path).expect("Failed to remove existing project directory");
    }

    // Run the generator command
    let output = Command::new("./target/debug/ts_microservice_generator")
        .arg("-n")
        .arg(project_name)
        .arg("-v")
        .arg("1.0")
        .arg("-a")
        .arg("John Doe")
        .arg("-b")
        .arg("This is a test project")
        .output()
        .expect("Failed to execute command");

    // Check the command output
    assert!(output.status.success());
    assert!(project_path.exists());

    // Verify that expected files are created
    assert!(project_path.join("package.json").exists());
    assert!(project_path.join("tsconfig.json").exists());
    assert!(project_path.join("src/index.ts").exists());

    // Clean up the generated project directory after the test
    fs::remove_dir_all(project_path).expect("Failed to remove test project directory");
}
