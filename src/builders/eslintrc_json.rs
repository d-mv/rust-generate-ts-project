use std::collections::HashMap;

#[derive(Debug)]
pub struct EslintrcJsonEnv {
    es2021: bool,
    node: bool,
    jest_globals: bool,
}

#[derive(Debug)]
pub struct EslintrcJsonParserOptions {
    ecmaVersion: u8,
    sourceType: String,
}

#[derive(Debug)]
pub struct EslintrcJson {
    env: EslintrcJsonEnv,
    extends: Vec<String>,
    parser: String,
    parserOptions: EslintrcJsonParserOptions,
    plugins: Vec<String>,
    // rules: HashMap<String, String>,
}

fn make_extends() -> Vec<String> {
    let extends: Vec<String> = vec![
        "eslint:recommended",
        "plugin:eslint-comments/recommended",
        "plugin:import/errors",
        "plugin:import/typescript",
        "plugin:import/warnings",
        "plugin:json/recommended",
        "plugin:jest/recommended",
        "plugin:markdown/recommended",
        "plugin:@typescript-eslint/eslint-recommended",
        "plugin:@typescript-eslint/recommended",
        "prettier",
        "plugin:prettier/recommended",
    ]
    .iter()
    .map(|el| String::from(el.to_owned()))
    .collect();

    extends
}

fn make_plugins() -> Vec<String> {
    let plugins: Vec<String> = vec![
        "@getify/proper-arrows",
        "@typescript-eslint",
        "async-await",
        "eslint-comments",
        "import",
        "jest",
        "prettier",
        "promise",
        "ramda",
    ]
    .iter()
    .map(|el| String::from(el.to_owned()))
    .collect();

    plugins
}

pub fn build_eslintrc_json() -> EslintrcJson {
    let env = EslintrcJsonEnv {
        es2021: true,
        node: true,
        jest_globals: true,
    };

    let parserOptions = EslintrcJsonParserOptions {
        ecmaVersion: 16,
        sourceType: String::from("module"),
    };
    let extends = make_extends();
	let plugins = make_plugins();
    EslintrcJson {
        env,
        extends,
        parser: String::from("@typescript-eslint/parser"),
        parserOptions,
        plugins,
        // rules: HashMap<String, String>,
    }
}
