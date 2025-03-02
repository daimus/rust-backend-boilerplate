use std::sync::Arc;
use crate::bootstrap::dependency::DependencyContainer;

#[derive(Clone)]
pub struct AppState {
    pub service_name: String,
    pub dependencies: Arc<DependencyContainer>
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            service_name: "my-service".to_string(),
            dependencies: Arc::new(DependencyContainer::new())
        }
    }
}