use std::sync::Arc;
use crate::application::todo::entity::todo::Todo;
use crate::application::todo::repository::todo_repository::TodoRepository;

pub struct GetTodosUsecase {
    repository: Arc<dyn TodoRepository + Send + Sync>,
}

impl GetTodosUsecase {
    pub fn new(repository: Arc<dyn TodoRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Vec<Todo> {
        self.repository.get_todos().await
    }
}