use async_trait::async_trait;
use crate::domain::entity::Customer;

#[async_trait]
pub trait CustomerRepository {
    async fn find_by_id(&self, id: i32) -> Result<Customer, Error>;
    async fn update(&self, customer: &Customer) -> Result<(), Error>;
    async fn delete(&self, id: i32) -> Result<(), Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Customer not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
} 