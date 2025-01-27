#[derive(Debug, Clone)]
pub struct Payment {
    pub id: String,
    pub amount: i64,
    pub currency: String,
}
