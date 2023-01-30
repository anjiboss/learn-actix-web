use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct IncrementData {
    pub times: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DecrementData {
    pub times: i32,
}
