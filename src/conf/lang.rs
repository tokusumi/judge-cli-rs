use super::config::{Config, LangTemplate};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn apply_py_template(
    path: &Path,
    stem: &str,
    lang_template: &Option<&LangTemplate>,
) -> Result<String, String> {
    let file_path = path.join(format!("{}.py", stem));
    if file_path.exists() {
        return Err(String::from("already exists"));
    } else {
        let mut file = File::create(&file_path).unwrap();
        let _temp = lang_template
            .map(|s| s.src.clone())
            .unwrap_or(String::from(""));
        write!(file, "{}", _temp)
            .map_err(|e| e.to_string())
            .unwrap();
    }
    Ok(String::from(file_path.as_path().to_str().unwrap()))
}
