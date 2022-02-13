use std::env;
use std::fs;
use std::path::Path;

use super::conf::config::TemplateConf;

pub fn command(path: Option<String>) {
    let path = match path {
        Some(s) => {
            let p = Path::new(&s);
            fs::create_dir_all(p).unwrap();
            p.to_path_buf()
        }
        None => env::current_dir().unwrap(),
    };
    TemplateConf::create_user_template(&path);
}
