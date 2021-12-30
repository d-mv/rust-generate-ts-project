#[derive(Debug)]
pub struct Depcheck {
    pub ignores: Vec<String>,
    pub skip_missing: bool,
}

impl Depcheck {
    fn to_file(&self) -> String {
        let arr: Vec<String> = self
            .ignores
            .iter()
            .map(|el| format!("\"{}\"", el))
            .collect();

        format!(
            "ignores: [{}]\nskip-missing: {}",
            arr.join(", "),
            &self.skip_missing
        )
    }
}

pub fn build_depcheck(additional_items: &[String]) -> Depcheck {
    let mut ignores: Vec<String> = ["eslint".to_string(), "husky".to_string()].into();
    ignores.extend(additional_items.iter().map(String::from));
    Depcheck {
        ignores,
        skip_missing: true,
    }
}
