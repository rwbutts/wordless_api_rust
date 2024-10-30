use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub alive: bool,
}
impl HealthCheckResponse { pub fn new() -> Self { Self {alive: true}}}

#[derive(Serialize)]
pub struct GetWordResponse {
    pub word: String,
}
impl GetWordResponse { pub fn new(word: String) -> Self { Self {word: word}}}

#[derive(Serialize)]
pub struct WordExistsResponse {
    pub exists: bool,
}
impl WordExistsResponse { pub fn new(exists: bool) -> Self { Self {exists: exists}}}

#[derive(Serialize)]
pub struct QueryMatchCountResponse {
    pub count: u32
}
impl QueryMatchCountResponse { pub fn new(count: u32) -> Self { Self {count: count}}}
