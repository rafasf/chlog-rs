extern crate regex;
// TODO:
//  * Test for multiple tag regex
//  * Remove tag from subject

use self::regex::Regex;

#[derive(Debug)]
struct Commit {
  tag: String,
  hash: String,
  subject: String,
  author: String
}

impl Commit {
  pub fn from(raw_commit: &str, separator: &str, tags_re: &Regex) -> Self {
    let raw_commit: Vec<&str> = raw_commit.split(separator).collect();

    let possible_tag = match tags_re.captures(raw_commit[0]) {
      Some(tag) => tag.get(1).unwrap().as_str(),
      None => ""
    };

    Commit {
      tag: possible_tag.to_string(),
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
    let no_re = Regex::new(r"^none").unwrap();
    let commit = Commit::from(
      "Sample message here-author-hash",
      "-",
      &no_re);
    let expected_commit = Commit {
      tag: "".to_string(),
      hash: "hash".to_string(),
      subject: "Sample message here".to_string(),
      author: "author".to_string()
    };

    assert_eq!(expected_commit.tag, commit.tag);
    assert_eq!(expected_commit.hash, commit.hash);
    assert_eq!(expected_commit.subject, commit.subject);
    assert_eq!(expected_commit.author, commit.author);
  }

  #[test]
  fn creates_commit_with_tag_from_string() {
    let tags_re = Regex::new(r"^(US\w+)").unwrap();
    let commit = Commit::from(
      "US123 Sample message here|author|hash",
      "|",
      &tags_re);

    let expected_commit = Commit {
      tag: "US123".to_string(),
      hash: "hash".to_string(),
      subject: "US123 Sample message here".to_string(),
      author: "author".to_string()
    };

    assert_eq!(expected_commit.tag, commit.tag);
    assert_eq!(expected_commit.hash, commit.hash);
    assert_eq!(expected_commit.subject, commit.subject);
    assert_eq!(expected_commit.author, commit.author);
  }
}

