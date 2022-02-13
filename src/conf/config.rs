use super::site::atocder_url;
use serde;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use toml;

const CONFIG_NAME: &str = ".judgecli";
const TEMPLATE_NAME: &str = ".judgecli-template";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub judgeconf: JudgeConf,
    pub template: TemplateConf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JudgeConf {
    pub workdir: String,
    pub testdir: String,
    pub URL: String,
    pub file: String,
    pub contest: String,
    pub problem: String,
    pub py: bool,
    pub pypy: bool,
    pub cython: bool,
    pub rs: bool,
    pub mle: f64,
    pub tle: f64,
    pub mode: String,
    pub verbose: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct JudgeConfTemplate {
    py: Option<bool>,
    pypy: Option<bool>,
    cython: Option<bool>,
    rs: Option<bool>,
    mle: f64,
    tle: f64,
    mode: Option<String>,
    verbose: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LangTemplate {
    pub src: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateConf {
    judgeconf: Option<JudgeConfTemplate>,
    pub py: Option<LangTemplate>,
    pub pypy: Option<LangTemplate>,
    pub cython: Option<LangTemplate>,
    pub rs: Option<LangTemplate>,
}

fn read_file(path: &Path) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

fn save_file(path: &Path, data: &str) -> Result<(), String> {
    let mut file = fs::File::create(&path).unwrap();
    write!(file, "{}", data);
    Ok(())
}

pub trait ConfigIO
where
    Self: DeserializeOwned + serde::Serialize,
{
    fn load(path: &Path) -> Result<Self, String>;
    fn save(&self, path: &Path) -> Result<(), String>;
    fn _load(path: &Path) -> Result<Self, String> {
        match read_file(path) {
            Ok(s) => match toml::from_str(&s) {
                Ok(p) => Ok(p),
                Err(e) => Err(format!("fail to parse toml: {}", e)),
            },
            Err(e) => Err(format!("fail to read file: {}", e)),
        }
    }
    fn _save(&self, path: &Path) -> Result<(), String> {
        match toml::to_string_pretty(self) {
            Ok(s) => save_file(&path, &s),
            Err(e) => panic!("failed to save toml: {}", e),
        }
    }
}

impl ConfigIO for JudgeConf {
    fn load(path: &Path) -> Result<Self, String> {
        let path = path.join(CONFIG_NAME);
        Self::_load(&path)
    }
    fn save(&self, path: &Path) -> Result<(), String> {
        let path = path.join(CONFIG_NAME);
        self._save(&path)
    }
}
impl ConfigIO for TemplateConf {
    fn load(path: &Path) -> Result<Self, String> {
        let path = path.join(TEMPLATE_NAME);
        Self::_load(&path)
    }
    fn save(&self, path: &Path) -> Result<(), String> {
        let path = path.join(TEMPLATE_NAME);
        self._save(&path)
    }
}

fn _fill<T>(u: &mut Option<T>, v: Option<T>) {
    if u.is_none() {
        *u = v
    };
}

impl JudgeConf {
    fn from_template(template: &JudgeConfTemplate) -> Self {
        Self {
            workdir: String::from(""),
            testdir: String::from(""),
            URL: String::from(""),
            file: String::from(""),
            contest: String::from(""),
            problem: String::from(""),
            py: template.py.unwrap(),
            pypy: template.pypy.unwrap(),
            cython: template.cython.unwrap(),
            rs: template.rs.unwrap(),
            mle: template.mle,
            tle: template.tle,
            mode: template.mode.as_ref().unwrap().clone(),
            verbose: template.verbose.as_ref().unwrap().clone(),
        }
    }
    fn system_conf() -> Self {
        Self {
            workdir: String::from(""),
            testdir: String::from(""),
            URL: String::from(""),
            file: String::from(""),
            contest: String::from(""),
            problem: String::from(""),
            py: true,
            pypy: false,
            cython: false,
            rs: false,
            mle: 1024.0,
            tle: 2000.0,
            mode: String::from("exact-match"),
            verbose: String::from("error_detail"),
        }
    }
}

impl JudgeConfTemplate {
    fn fill_none(&mut self, t: JudgeConfTemplate) {
        _fill(&mut self.py, t.py);
        _fill(&mut self.pypy, t.pypy);
        _fill(&mut self.cython, t.cython);
        _fill(&mut self.rs, t.rs);
        _fill(&mut self.mode, t.mode);
        _fill(&mut self.verbose, t.verbose);
    }
}

impl TemplateConf {
    pub fn create_user_template(path: &Path) -> Result<(), String> {
        let conf = Self::system_template();
        conf.save(&path)
    }
    fn fill_none(&mut self, t: TemplateConf) {
        if let Some(u) = &mut self.judgeconf {
            if let Some(v) = t.judgeconf {
                u.fill_none(v)
            }
        } else {
            _fill(&mut self.judgeconf, t.judgeconf)
        }
        _fill(&mut self.py, t.py);
        _fill(&mut self.pypy, t.pypy);
        _fill(&mut self.cython, t.cython);
        _fill(&mut self.rs, t.rs);
    }
    fn system_template() -> TemplateConf {
        TemplateConf {
            judgeconf: Some(JudgeConfTemplate {
                py: Some(true),
                pypy: Some(false),
                cython: Some(false),
                rs: Some(false),
                mle: 1024.0,
                tle: 2000.0,
                mode: Some(String::from("exact-match")),
                verbose: Some(String::from("error_detail")),
            }),
            py: Some(LangTemplate {
                src: String::from(
                    r#"
def main():
    return


if __name__=="__main__":
    ans = main()
    print(ans)
"#,
                ),
            }),
            pypy: None,
            cython: None,
            rs: None,
        }
    }
}

impl Config {
    pub fn new(cwd: &Path, contest: Option<String>, problem: Option<String>) -> Self {
        // local config
        let mut user_template_conf = None;
        let mut n: Option<&Path> = Some(cwd);
        while let Some(p) = n {
            user_template_conf = TemplateConf::load(p).ok();
            if user_template_conf.is_some() {
                break;
            }
            n = p.parent();
        }

        // global config

        // system config
        let sys_template = TemplateConf::system_template();
        match &mut user_template_conf {
            Some(l) => l.fill_none(sys_template),
            None => user_template_conf = Some(sys_template),
        }

        // convert temp into judge conf
        let mut judge_conf = if let Some(v) = &user_template_conf {
            if let Some(x) = &v.judgeconf {
                JudgeConf::from_template(x)
            } else {
                JudgeConf::system_conf()
            }
        } else {
            JudgeConf::system_conf()
        };
        // add values
        judge_conf.contest = contest.unwrap_or(String::from(""));
        judge_conf.problem = problem.unwrap_or(String::from(""));
        if judge_conf.contest != "" && judge_conf.problem != "" {
            judge_conf.URL = atocder_url(&judge_conf.contest, &judge_conf.problem);
        }
        judge_conf.workdir = cwd
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or(String::from(""));
        judge_conf.testdir = cwd
            .join("tests")
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or(String::from(""));
        Self {
            judgeconf: judge_conf,
            template: user_template_conf.unwrap(),
        }
    }

    pub fn create_user_config(&self, path: &Path) -> Result<(), String> {
        if let Err(e) = self.judgeconf.save(&path) {
            panic!("failed to save: {}", e);
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fill() {
        // none does not override value
        let mut a = Some("abc");
        let b = None;
        _fill(&mut a, b);
        assert_eq!(a, Some("abc"));

        // override none
        let mut a = None;
        let b = Some("abc");
        _fill(&mut a, b);
        assert_eq!(a, Some("abc"));

        // value is not overrided
        let mut a = Some("abc");
        let b = Some("xyz");
        _fill(&mut a, b);
        assert_eq!(a, Some("abc"));

        // none vs none => nothing happens
        let mut a: Option<bool> = None;
        let b = None;
        _fill(&mut a, b);
        assert_eq!(a, None);
    }
}
