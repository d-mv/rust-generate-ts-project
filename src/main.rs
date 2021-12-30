use crate::builders::config::make_config;
use crate::builders::depcheck::build_depcheck;
use crate::builders::eslintrc_json::build_eslintrc_json;
use crate::builders::package_json::build_package_json;

mod builders;

fn main() {
    let config = make_config();
    println!("config: {:#?}", config);
    let package_json = build_package_json(&config);
    println!("package.json: {:#?}", package_json);
    let a: Vec<String> = vec![String::from("webpack")];
    let depcheck = build_depcheck(&a);
    println!("depcheck: {:#?}", depcheck);
    let eslintrc_json = build_eslintrc_json();
    println!("eslintrc.json: {:#?}", eslintrc_json);
}
