use crate::domain::customer::Customer;
use crate::domain::repository::{CustomerRepository, Error};

pub struct CustomerUseCase<R: CustomerRepository> {
    repository: R,
}

impl<R: CustomerRepository> CustomerUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_customer(&self, id: i32) -> Result<Customer, Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn update_customer(&self, customer: &Customer) -> Result<(), Error> {
        self.repository.update(customer).await
    }

    pub async fn delete_customer(&self, id: i32) -> Result<(), Error> {
        self.repository.delete(id).await
    }
} 