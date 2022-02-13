use std::fs;
use std::path::Path;

pub fn command(testdir: &str) {
    // Check or create test directory
    if testdir == "" {
        println!("No test directory configuration");
        return;
    }
    let path = Path::new(testdir);
    let res = fs::create_dir_all(&path);
    if res.is_err() {
        println!("Can't create test directory");
        return;
    }
    // search the latest sample number
    let mut inc = 0;
    let idx = loop {
        inc += 1;
        let in_path = path.join(format!("sample-{}.in", inc));
        if !in_path.exists() {
            break inc;
        }
    };
    // create new brank test files
    let in_res = fs::File::create(path.join(format!("sample-{}.in", idx)));
    let out_res = fs::File::create(path.join(format!("sample-{}.out", idx)));
    if in_res.is_ok() && out_res.is_ok() {
        println!("add: sample-{}.in/out", idx);
    } else {
        println!("Can't add new brank test case files: sample-{}.in/out", idx);
    }
}
