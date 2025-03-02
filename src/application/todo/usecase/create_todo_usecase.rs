use std::sync::Arc;
use crate::application::todo::entity::todo::{CreateTodo, Todo};
use crate::application::todo::repository::todo_repository::TodoRepository;

pub struct CreateTodosUsecase {
    repository: Arc<dyn TodoRepository + Send + Sync>,
}

impl CreateTodosUsecase {
    pub fn new(repository: Arc<dyn TodoRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, todo: CreateTodo) -> Todo {
        self.repository.create_todo(todo).await
    }
}