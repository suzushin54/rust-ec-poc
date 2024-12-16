use serde::Deserialize;

#[derive(Deserialize)]
pub struct Payment {
    pub amount: u64,
    pub currency: String,
}
