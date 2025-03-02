use std::sync::Arc;
use crate::application::todo::entity::todo::{Todo, UpdateTodo};
use crate::application::todo::repository::todo_repository::TodoRepository;
use crate::common::app_error::AppError;

pub struct UpdateTodoUsecase {
    repository: Arc<dyn TodoRepository + Send + Sync>,
}

impl UpdateTodoUsecase {
    pub fn new(repository: Arc<dyn TodoRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, todo_id: i32, todo: UpdateTodo) -> Result<Todo, AppError> {
        let todo_to_update = self.repository.get_todo(todo_id).await;
        if todo_to_update.is_none() {
            return Err(AppError::NotFound);
        }
        Ok(self.repository.update_todo(todo_id, todo).await)
    }
}