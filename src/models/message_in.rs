#[derive(Debug, serde::Deserialize)]
pub struct MessageIn {
    text: String,
    room: String,
}
