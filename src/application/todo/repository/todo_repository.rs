use async_trait::async_trait;
use crate::application::todo::entity::todo::{CreateTodo, Todo, UpdateTodo};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_todos(&self) -> Vec<Todo>;
    async fn get_todo(&self, todo_id: i32) -> Option<Todo>;
    async fn create_todo(&self, todo: CreateTodo) -> Todo;
    async fn update_todo(&self, todo_id: i32, todo: UpdateTodo) -> Todo;
    async fn delete_todo(&self, todo_id: i32);
}