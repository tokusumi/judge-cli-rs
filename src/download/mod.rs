use serde_derive::Deserialize;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Tests {
    input: String,
    output: String,
}
#[derive(Debug, Deserialize)]
pub struct Problem {
    pub memoryLimit: usize,
    pub timeLimit: usize,
    tests: Vec<Tests>,
}

#[derive(Debug, Deserialize)]
struct ExecStdOut<T> {
    status: String,
    messages: Vec<String>,
    result: T,
}

pub fn command(url: &str, testdir: &Path) {
    // download
    let _out = retrieve_samples(url);
    if _out.is_err() {
        println!("{:?}", _out);
        return;
    };
    let out = _out.unwrap();

    // testcases, mle, tle

    // save them
    // TODO: add error handling
    out.dump_samples(testdir);
}

pub fn retrieve_samples(url: &str) -> Result<Problem, String> {
    // download with oj-api
    let output = Command::new("oj-api")
        .args(&["get-problem", url])
        .output()
        .expect("failed to download");
    // parse output
    let output = output.stdout;
    let data: Result<ExecStdOut<Problem>, _> = serde_json::from_slice(&output);
    match data {
        Ok(v) => Ok(v.result),
        Err(e) => Err(e.to_string()),
    }
}

impl Problem {
    pub fn dump_samples(&self, testdir: &Path) {
        fs::create_dir_all(testdir).unwrap();
        println!("success");
        let mut inc = 0;
        for t in &self.tests {
            inc += 1;
            let p = testdir.join(format!("sample-{}.in", inc));
            fs::write(p, &t.input);
            let p = testdir.join(format!("sample-{}.out", inc));
            fs::write(p, &t.output);
        }
    }
}
