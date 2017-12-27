use changelog::Changelog;

pub fn create_from(changelog: &Changelog) {
  println!("## {:?}, {}", changelog.title, changelog.created_at);

  for (tag, commits) in changelog.commits_by_tag() {
    println!("### {}", tag);
    commits.iter().for_each(|commit| println!("* {} ({})", commit.subject, commit.hash))
  }
}
