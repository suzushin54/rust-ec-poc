pub mod domain;
pub mod adapters;
pub mod application;
pub mod handlers;

pub use domain::repository::customer_repository::CustomerRepository;
pub use adapters::repository::customer_repository::CustomerRepositoryImpl;
pub use handlers::customer_handler::CustomerHandler;
pub use application::usecase::customer_usecase::CustomerUseCase;

pub fn init() {
    println!("Management module initialized");
} 