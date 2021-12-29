use crate::builders::package_json::build_package_json;
use crate::config::make_config;

mod builders;
mod config;

fn main() {
    let config = make_config();
    println!("config: {:#?}", config);
    let package_json = build_package_json(&config);
    println!("package.json: {:#?}", package_json);
}
