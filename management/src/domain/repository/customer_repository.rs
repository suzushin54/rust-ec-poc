use async_trait::async_trait;
use crate::domain::entity::customer::Customer;

#[async_trait]
pub trait CustomerRepository {
    async fn find_by_id(&self, id: i32) -> Option<Customer>;
    async fn update(&self, customer: &Customer);
    async fn delete(&self, id: i32);
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Customer not found")]
    NotFound,
    #[error("Database error")]
    Database(String),
} 