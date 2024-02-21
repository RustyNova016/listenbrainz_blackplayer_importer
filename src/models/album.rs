use super::artist::Artist;

pub struct Album {
    msid: Option<String>,
    name: String,
    author: Option<Artist>
}