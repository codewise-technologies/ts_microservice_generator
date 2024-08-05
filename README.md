
# TypeScript Microservice Generator

A command-line tool to generate a boilerplate for TypeScript microservices.

## Installation

To use the TypeScript Microservice Generator, you need to have Rust and Cargo installed. Clone this repository and build the project:

```bash
git clone https://github.com/codewise-technologies/ts_microservice_generator.git
cd ts_microservice_generator
cargo build --release
```

Alternatively, you can install the tool directly using Cargo:

```bash
cargo install ts_microservice_generator
```

This will download and install the `ts_microservice_generator` crate from [crates.io](https://crates.io/) and make it available as a global command on your system.

## Usage

After installation, you can use the generator to create a new TypeScript microservice boilerplate. Use the following command format:

```bash
ts_microservice_generator -n <project_name> -v <version> -a <author> -b <description> -t <architecture>
```

Where `<architecture>` can be one of the following:
- `hexagonal`
- `clean_code`
- `saga`

### Example

To create a project with a hexagonal architecture:

```bash
ts_microservice_generator -n MyProject -v 2.0 -a "John Doe" -b "This is a TypeScript microservice example." -t hexagonal
```

Output:

```
Project MyProject generated successfully with hexagonal architecture
```

To create a project with a clean code architecture:

```bash
ts_microservice_generator -n MyProject -v 2.0 -a "John Doe" -b "This is a TypeScript microservice example." -t clean_code
```

Output:

```
Project MyProject generated successfully with clean code architecture
```

To create a project with a saga architecture:

```bash
ts_microservice_generator -n MyProject -v 2.0 -a "John Doe" -b "This is a TypeScript microservice example." -t saga
```

Output:

```
Project MyProject generated successfully with saga architecture
```

In these examples, the generator will create a new project named `MyProject` with version `2.0`, authored by "John Doe," and described as "This is a TypeScript microservice example." The generated project will include all the necessary files and structure to start developing a TypeScript microservice with the specified architecture.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
