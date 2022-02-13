use clap::ArgEnum;
use std::env;
use std::fs;
use std::path::Path;
pub mod config;
pub mod lang;
pub mod site;
use super::download::retrieve_samples;
use config::ConfigIO;

#[derive(ArgEnum, Clone, Debug)]
pub enum Lang {
    Py,
    PyPy,
    Rs,
}

pub fn command(contest: Option<String>, problem: Option<String>, ii: bool, lang: Option<Lang>) {
    // TODO: support interactive mode
    let dirname = format!(
        "{}_{}",
        contest.as_ref().unwrap_or(&"contest".to_string()),
        problem.as_ref().unwrap_or(&"".to_string())
    );
    let stem = format!("{}", problem.as_ref().unwrap_or(&"sol".to_string()));

    // detemine working directory
    let cwd = env::current_dir().unwrap().join(dirname);
    if cwd.exists() {
        panic!("already exists")
    } else {
        fs::create_dir_all(&cwd).unwrap();
    }

    // load and override config
    let mut _conf = config::Config::new(&cwd, contest, problem);

    // download system test cases
    // make TLE and MLE compatible with problem
    if _conf.judgeconf.URL != "" {
        let _out = retrieve_samples(&_conf.judgeconf.URL);
        if _out.is_err() {
            println!("{:?}", _out);
            return;
        };
        let out = _out.unwrap();
        out.dump_samples(&Path::new(&_conf.judgeconf.testdir));
        _conf.judgeconf.mle = out.memoryLimit as f64;
        _conf.judgeconf.tle = out.timeLimit as f64;
    }

    // create codebase from template
    let _file_path: Result<String, String> = match lang.unwrap_or(Lang::Py) {
        Lang::Py => lang::apply_py_template(&cwd, &stem, &_conf.template.py.as_ref()),
        Lang::PyPy => unreachable!(),
        Lang::Rs => unreachable!(),
    };
    // update target file path in user judge config
    if let Some(file_path) = _file_path.ok() {
        _conf.judgeconf.file = file_path;
    }

    // save user config
    _conf.create_user_config(&cwd);
}
