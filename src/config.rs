use std::env;
use std::env::args;

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub current_dir: Option<String>,
    pub with_latest: bool,
    pub folders: Vec<String>,
    pub builder: Option<String>,
    pub keywords: Vec<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub description: Option<String>,
}

fn get_argument_index(arguments: &[String], keyword: &str) -> Option<usize> {
    let argument_index = &arguments.iter().position(|r| r == keyword);
    argument_index
        .as_ref()
        .map(|index| index.to_owned())
        .to_owned()
}

fn get_argument_by_keyword(arguments: &[String], keyword: &str) -> Option<String> {
    let folder_index = get_argument_index(arguments, keyword);
    match folder_index {
        Some(index) => {
            let folders = &arguments.get(index.to_owned() + 1);
            if folders.is_some() {
                return Some(folders.unwrap().to_owned());
            }
            None
        }
        None => None,
    }
}

fn get_folders(arguments: &[String]) -> Vec<String> {
    let argument = get_argument_by_keyword(arguments, "--folders");
    match argument {
        Some(a) => a.split(';').map(String::from).collect(),
        None => vec![],
    }
}

fn get_keywords(arguments: &[String]) -> Vec<String> {
    let argument = get_argument_by_keyword(arguments, "--keywords");
    match argument {
        Some(a) => a.split(';').map(String::from).collect(),
        None => vec![],
    }
}

fn get_builder(arguments: &[String]) -> Option<String> {
    get_argument_by_keyword(arguments, "--builder")
}

fn get_author(arguments: &[String]) -> Option<String> {
    get_argument_by_keyword(arguments, "--author")
}

fn get_license(arguments: &[String]) -> Option<String> {
    get_argument_by_keyword(arguments, "--license")
}

fn get_description(arguments: &[String]) -> Option<String> {
    get_argument_by_keyword(arguments, "--description")
}

fn get_with_latest(arguments: &[String]) -> bool {
    get_argument_index(arguments, "--latest").is_some()
}

fn get_current_dir() -> Option<String> {
    let maybe_current_dir = env::current_dir();
    match maybe_current_dir {
        Ok(current_dir) => current_dir.to_str().map(String::from),
        Err(err) => {
            eprintln!("Error on getting current dir: {:#?}", err);
            None
        }
    }
}

pub fn make_config() -> Config {
    let arguments: Vec<String> = args().collect();

    Config {
        name: String::from(&arguments[1]),
        current_dir: get_current_dir(),
        with_latest: get_with_latest(&arguments),
        folders: get_folders(&arguments),
        builder: get_builder(&arguments),
        keywords: get_keywords(&arguments),
        author: get_author(&arguments),
        license: get_license(&arguments),
        description: get_description(&arguments),
    }
}
