// TODO:
//  * Extract tag from subject based on a set of regex
//  * Allow different separators (assume same per run)

#[derive(Debug)]
struct Commit {
    hash: String,
    subject: String,
    author: String
}

impl Commit {
    pub fn from(raw_commit: &str) -> Self {
        let raw_commit: Vec<&str> = raw_commit.split("|").collect();
        Commit {
            hash: raw_commit[2].to_string(),
            subject: raw_commit[0].to_string(),
            author: raw_commit[1].to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use commit::*;

    #[test]
    fn creates_commit_from_string() {
        let commit = Commit::from("Sample message here|author|hash");
        let expected_commit = Commit {
            hash: "hash".to_string(),
            subject: "Sample message here".to_string(),
            author: "author".to_string()
        };

        assert_eq!(expected_commit.hash, commit.hash);
        assert_eq!(expected_commit.subject, commit.subject);
        assert_eq!(expected_commit.author, commit.author);
    }
}

