use show::*;
use std::process::Command;

pub mod changelog;
pub mod commit;
pub mod new_commit;
pub mod tag;

pub struct LogMessage {
    pub subject: String,
    pub author: String,
    pub hash: String,
    pub body: String,
}

pub fn fetch_log(repository_dir: &str, format: &str, range: &str) -> Vec<String> {
    let git_dir = if repository_dir.contains(".git") {
        repository_dir.to_string()
    } else {
        format!("{}/.git", repository_dir)
    };

    let commit_hashes = Command::new("git")
        .arg("--git-dir")
        .arg(&git_dir)
        .arg("log")
        .arg("--no-merges")
        .arg("--pretty=format:%H")
        .arg(range)
        .output()
        .unwrap()
        .stdout;

    String::from_utf8_lossy(&commit_hashes)
        .lines()
        .map(|hash| commit_info_for(hash, &git_dir, format))
        .collect()
}

fn commit_info_for(hash: &str, git_dir: &str, format: &str) -> String {
    let possible_log = Command::new("git")
        .arg("--git-dir")
        .arg(&git_dir)
        .arg("log")
        .arg(hash)
        .arg(format)
        .arg("--max-count=1")
        .output()
        .expect("Failed to interact with Git.");

    String::from_utf8_lossy(&possible_log.stdout).into_owned()
}
