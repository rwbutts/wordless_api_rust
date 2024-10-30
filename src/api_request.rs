use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryMatchCountRequest {
    pub answer: String,
    pub guesses: Vec<String>
}

