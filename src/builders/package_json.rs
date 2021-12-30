use std::collections::HashMap;

use crate::builders::config::Config;

type Scripts = HashMap<&'static str, &'static str>;

// dependencies will be installed via npm i
#[derive(Debug)]
pub struct PackageJson {
    name: String,
    version: String,
    description: String,
    main: String,
    scripts: Scripts,
    keywords: Vec<String>,
    author: String,
    license: String,
}

pub fn build_package_json(config: &Config) -> PackageJson {
    println!("building");
    let empty = String::from("");
    let description = &config.description.as_ref().unwrap_or(&empty);
    let keywords = Vec::to_owned(&config.keywords);
    let license = &config.license.as_ref().unwrap_or(&empty);
    let author = &config.author.as_ref().unwrap_or(&empty);

    PackageJson {
        name: String::from(&config.name),
        version: String::from("0.1.0"),
        description: description.to_string(),
        main: String::from("index.ts"),
        scripts: make_scripts(),
        keywords,
        license: license.to_string(),
        author: author.to_string(),
    }
}

pub fn make_scripts() -> Scripts {
    let mut h: Scripts = HashMap::new();

    h.insert("lint", "eslint --ext .ts,.tsx src");
    h.insert(
        "test:ci",
        "CI=true NODE_ENV=test TZ='UTC' jest --config ./jest.config.js --passWithNoTests",
    );
    h.insert("test", "run-p test:typescript test:dependencies test:ci");
    h.insert("test:dependencies", "npx depcheck");
    h.insert(
        "test:complete",
        "npm run test:typescript && npm run test && npm run test:dependencies",
    );
    h.insert("test:typescript", "tsc --noEmit -p ./tsconfig.json");
    h.insert(
        "test:watch:coverage",
        "NODE_ENV=test jest --coverage --watchAll",
    );
    h.insert(
        "test:watch",
        "NODE_ENV=test TZ='UTC' jest --watchAll --detectOpenHandles",
    );
    h.insert("prepare", "husky install");
    h.insert("start", "NODE_ENV=production node dist/index.js");
    h.insert("build", "rm -rf dist && webpack");
    h.insert("dev", "NODE_ENV=development ts-node-dev src/index.ts");
    h.insert(
        "docker:build",
        "docker build --pull --rm -f Dockerfile.dev -t new-repo .",
    );
    h.insert("docker:run", "docker run --rm -it --network=host new-repo");
    h
}
