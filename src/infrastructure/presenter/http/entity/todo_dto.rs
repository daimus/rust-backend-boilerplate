use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::application::todo::entity::todo::{CreateTodo, Todo, UpdateTodo};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct TodoDto {
    pub id: i32,
    pub activity: String,
    pub completed: bool,
}
pub struct TodoList(pub Vec<Todo>);

impl From<TodoList> for Vec<TodoDto> {
    fn from(todo_models: TodoList) -> Self {
        todo_models.0.into_iter().map(TodoDto::from).collect()
    }
}
impl From<Todo> for TodoDto {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            activity: todo.activity,
            completed: todo.completed,
        }
    }
}
pub struct TodoOption(pub Option<Todo>);

impl From<TodoOption> for Option<TodoDto> {
    fn from(opt_model: TodoOption) -> Self {
        opt_model.0.map(TodoDto::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateTodoDto {
    pub activity: String,
}
impl From<CreateTodoDto> for CreateTodo {
    fn from(todo: CreateTodoDto) -> Self {
        Self {
            activity: todo.activity
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoDto {
    pub activity: String,
    pub completed: bool,
}
impl From<UpdateTodoDto> for UpdateTodo {
    fn from(todo: UpdateTodoDto) -> Self {
        Self {
            activity: todo.activity,
            completed: todo.completed
        }
    }
}