#[derive(Debug)]
pub struct Story {
    pub id: String,
    pub name: Option<String>,
    pub link: Option<String>,
}

impl Story {
    pub fn new<T: Into<String>>(id: T, name: Option<String>, link: Option<String>) -> Story {
        Story {
            id: id.into(),
            name: name,
            link: link,
        }
    }

    pub fn only_with<T>(id: T) -> Story
    where
        T: Into<String>,
    {
        Story::new(id.into(), None, None)
    }
}
