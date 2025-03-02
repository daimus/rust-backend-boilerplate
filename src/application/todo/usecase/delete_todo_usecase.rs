use std::sync::Arc;
use crate::application::todo::repository::todo_repository::TodoRepository;

pub struct DeleteTodoUsecase {
    repository: Arc<dyn TodoRepository + Send + Sync>,
}

impl DeleteTodoUsecase {
    pub fn new(repository: Arc<dyn TodoRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i32) {
        self.repository.delete_todo(id).await
    }
}