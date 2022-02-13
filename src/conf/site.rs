pub fn atocder_url(contest: &str, problem: &str) -> String {
    let s = format!(
        "https://atcoder.jp/contests/{}/tasks/{}_{}",
        contest, contest, problem
    );
    String::from(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_atcoder_url() {
        assert_eq!(
            atocder_url("abc", "a"),
            String::from("https://atcoder.jp/contests/abc/tasks/abc_c")
        );
    }
}
