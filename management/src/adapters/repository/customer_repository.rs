use async_trait::async_trait;
use crate::domain::entity::customer::Customer;
use crate::domain::repository::customer_repository::CustomerRepository;

pub struct CustomerRepositoryImpl {
    // DB connection等の必要な依存関係をここに
}

#[async_trait]
impl CustomerRepository for CustomerRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Option<Customer> {
        todo!()
    }

    async fn update(&self, customer: &Customer) {
        todo!()
    }

    async fn delete(&self, id: i32) {
        todo!()
    }
} 