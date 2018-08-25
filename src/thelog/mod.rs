use show::*;
use std::process::{Command, Output};

pub mod changelog;
pub mod commit;

pub fn fetch_log(repository_dir: &str, format: &str, range: &str) -> Output {
    let git_dir = if repository_dir.contains(".git") {
        repository_dir.to_string()
    } else {
        format!("{}/.git", repository_dir)
    };

    let possible_log = Command::new("git")
        .arg("--git-dir")
        .arg(&git_dir)
        .arg("log")
        .arg("--oneline")
        .arg("--no-merges")
        .arg(format)
        .arg(range)
        .output()
        .expect("Failed to interact with Git.");

    if possible_log.status.success() {
        possible_log
    } else {
        show_err(
            "Unable to get commits, please check the information provided.".to_string(),
        );
        show_err(format!("Repository: {}, range: {}", &git_dir, range));
        panic!();
    }
}
