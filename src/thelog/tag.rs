extern crate regex;

use self::regex::Regex;

#[derive(Clone, Debug)]
pub struct Tag {
    re: Regex,
    description: String,
}

impl Tag {
    pub fn from(pattern: &str, description: &str) -> Self {
        Tag {
            re: Regex::new(pattern).unwrap(),
            description: description.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TagMatch {
    tag: Tag,
    component: Option<String>,
}

pub fn tag_in(text: &str, tags: &Vec<Tag>) -> TagMatch {
    tags.iter()
        .filter_map(|tag| tag_and_component_given(tag, text))
        .last()
        .unwrap()
}

fn tag_and_component_given(a_tag: &Tag, text: &str) -> Option<TagMatch> {
    match a_tag.re.captures(text) {
        Some(matches) => Some(TagMatch {
            tag: a_tag.clone(),
            component: matches.get(1).map_or(None, |m| Some(m.as_str().into())),
        }),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use thelog::tag::*;

    #[test]
    fn creates_tag_from_pattern() {
        let pattern = r"chore\((\w+)\):\s*";
        let tag = Tag::from(pattern, "Chore");

        assert_eq!(pattern, tag.re.as_str());
    }

    #[test]
    #[should_panic]
    fn fails_when_tag_pattern_is_invalid() {
        Tag::from(r"(chore:\*", "None");
    }

    #[test]
    fn tag_match_has_component() {
        let pattern = r"chore\((\w+)\)\s*";
        let tag = Tag::from(pattern, "Chore");

        let my_tag = tag_in("My awesome commit message chore(ci)", &vec![tag]);

        assert_eq!(my_tag.tag.description, "Chore");
        assert_eq!(my_tag.component, Some("ci".into()));
    }

    #[test]
    fn tag_match_does_not_have_component_for_simple_tag() {
        let pattern = r"refactor:\s*";
        let tag = Tag::from(pattern, "Refactor");

        let my_tag = tag_in("My awesome commit message refactor:", &vec![tag]);

        assert_eq!(my_tag.tag.description, "Refactor");
        assert_eq!(my_tag.component, None);
    }
}
