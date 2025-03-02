use std::sync::Arc;
use crate::application::todo::entity::todo::Todo;
use crate::application::todo::repository::todo_repository::TodoRepository;

pub struct GetTodoUsecase {
    repository: Arc<dyn TodoRepository + Send + Sync>,
}

impl GetTodoUsecase {
    pub fn new(repository: Arc<dyn TodoRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) -> Option<Todo> {
        self.repository.get_todo(id).await
    }
}