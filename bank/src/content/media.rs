#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}
impl Media {

    pub fn title(&self) -> String {
        match self {
            Media::Book { title, .. } => title.to_string(),
            Media::Movie { title, .. } => title.to_string(),
            Media::Audiobook { title } => title.to_string(),
            Media::Podcast(title) => format!("Podcast {}", title),
            Media::Placeholder => String::from("Placeholder"),
        }
    }
}