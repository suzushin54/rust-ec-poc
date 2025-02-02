use crate::application::usecase::customer_usecase::CustomerUseCase;
use crate::domain::repository::CustomerRepository;

pub struct CustomerHandler<R: CustomerRepository> {
    usecase: CustomerUseCase<R>,
}

impl<R: CustomerRepository> CustomerHandler<R> {
    pub fn new(usecase: CustomerUseCase<R>) -> Self {
        Self { usecase }
    }

    pub async fn get_customer(&self, id: i32) -> Result<String, String> {
        match self.usecase.get_customer(id).await {
            Some(customer) => Ok("Customer found".to_string()),
            None => Err("Customer not found".to_string()),
        }
    }

    pub async fn delete_customer(&self, id: i32) -> Result<String, String> {
        self.usecase.delete_customer(id).await;
        Ok("Customer deleted".to_string())
    }
} 