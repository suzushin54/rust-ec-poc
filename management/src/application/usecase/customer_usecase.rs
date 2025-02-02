use crate::domain::entity::Customer;
use crate::domain::repository::CustomerRepository;

pub struct CustomerUseCase<R: CustomerRepository> {
    repository: R,
}

impl<R: CustomerRepository> CustomerUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_customer(&self, id: i32) -> Option<Customer> {
        self.repository.find_by_id(id).await
    }

    pub async fn update_customer(&self, customer: &Customer) {
        self.repository.update(customer).await
    }

    pub async fn delete_customer(&self, id: i32) {
        self.repository.delete(id).await
    }
} 