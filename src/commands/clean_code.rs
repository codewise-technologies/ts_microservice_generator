use std::fs;
use std::path::Path;
use tera::{Context, Tera};
use crate::utils::fs_utils::create_dir_all_safe;

pub fn generate(project_name: String, version: String, author: String, about: String) {
    let project_path = Path::new(&project_name);

    if project_path.exists() {
        eprintln!("The directory {} already exists", project_name);
        return;
    }

    create_dir_all_safe(project_path).expect("Failed to create project directory");

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/clean_code/**/*"))
        .expect("Error loading templates");

    let mut context = Context::new();
    context.insert("project_name", &project_name);
    context.insert("project_version", &version);
    context.insert("project_author", &author);
    context.insert("project_description", &about);

    let files = vec!["package.json.tera", "tsconfig.json.tera"];
    for file in files {
        let rendered = tera.render(file, &context).expect("Error rendering template");
        let file_name = file.trim_end_matches(".tera");
        let file_path = project_path.join(file_name);
        fs::write(file_path, rendered).expect("Error writing file");
    }

    let dirs = vec![
        "src",
        "src/config",
        "src/controllers",
        "src/domain",
        "src/domain/entities",
        "src/domain/interfaces",
        "src/domain/services",
        "src/infrastructure",
        "src/infrastructure/adapters",
        "src/infrastructure/repositories"
    ];
    for dir in dirs {
        create_dir_all_safe(project_path.join(dir)).expect("Error creating directory");
    }

    let files_content = vec![
        ("src/app.ts", r#"import express from 'express';
import dotenv from 'dotenv';
import { ExampleController } from './controllers/example.controller';

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

app.use('/example', ExampleController);

app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
"#),
        ("src/config/index.ts", r#"// Configuration settings
export const config = {
    port: process.env.PORT || 3000,
};
"#),
        ("src/controllers/example.controller.ts", r#"import { Router } from 'express';

const router = Router();

router.get('/', (req, res) => {
    res.send('Hello from Example Controller');
});

export const ExampleController = router;
"#),
        ("src/domain/entities/example.entity.ts", r#"// Example entity
export class Example {
    constructor(public id: number, public name: string) {}
}
"#),
        ("src/domain/interfaces/example.interface.ts", r#"// Example interface
export interface IExampleService {
    getExample(id: number): Example;
}
"#),
        ("src/domain/services/example.service.ts", r#"import { IExampleService } from '../interfaces/example.interface';
import { Example } from '../entities/example.entity';

export class ExampleService implements IExampleService {
    getExample(id: number): Example {
        return new Example(id, 'Example Name');
    }
}
"#),
        ("src/infrastructure/adapters/example.adapter.ts", r#"// Example adapter
export class ExampleAdapter {
    // Adapter logic
}
"#),
        ("src/infrastructure/repositories/example.repository.ts", r#"// Example repository
export class ExampleRepository {
    // Repository logic
}
"#),
        ("src/index.ts", r#"import express from 'express';
import dotenv from 'dotenv';
import { ExampleController } from './controllers/example.controller';

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

app.use('/example', ExampleController);

app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
"#)
    ];

    for (path, content) in files_content {
        let file_path = project_path.join(path);
        fs::write(file_path, content).expect("Error writing file");
    }

    println!("Project {} generated successfully with clean code architecture", project_name);
}
