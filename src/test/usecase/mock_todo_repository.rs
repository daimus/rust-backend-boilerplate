use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::application::todo::entity::todo::{CreateTodo, Todo, UpdateTodo};
use crate::application::todo::repository::todo_repository::TodoRepository;

pub struct MockTodoRepository {
    pub get_todo: Arc<Mutex<Option<Todo>>>,
    pub get_todos: Arc<Mutex<Vec<Todo>>>,
    pub create_todo: Arc<Mutex<Todo>>,
    pub update_todo: Arc<Mutex<Todo>>,
}

impl MockTodoRepository {
    pub fn new() -> Self {
        Self {
            get_todo: Arc::new(Mutex::new(Some(Todo{
                id: 1,
                activity: "activity1".to_string(),
                completed: true,
            }))),
            get_todos: Arc::new(Mutex::new(vec![
                Todo {
                    id: 1,
                    activity: "activity1".to_string(),
                    completed: true,
                },
                Todo {
                    id: 2,
                    activity: "activity2".to_string(),
                    completed: false,
                },
                Todo {
                    id: 3,
                    activity: "activity3".to_string(),
                    completed: false,
                }
            ])),
            create_todo: Arc::new(Mutex::new(Todo{
                id: 1,
                activity: "activity1".to_string(),
                completed: true,
            })),
            update_todo: Arc::new(Mutex::new(Todo{
                id: 1,
                activity: "activity1".to_string(),
                completed: true,
            })),
        }
    }
}

#[async_trait]
impl TodoRepository for MockTodoRepository {
    async fn get_todos(&self) -> Vec<Todo> {
        self.get_todos.lock().unwrap().clone()
    }

    async fn get_todo(&self, _todo_id: i32) -> Option<Todo> {
        if _todo_id == 2 {
            return None
        }
        self.get_todo.lock().unwrap().clone()
    }

    async fn create_todo(&self, _todo: CreateTodo) -> Todo {
        self.create_todo.lock().unwrap().clone()
    }

    async fn update_todo(&self, _todo_id: i32, _todo: UpdateTodo) -> Todo {
        self.update_todo.lock().unwrap().clone()
    }

    async fn delete_todo(&self, _todo_id: i32) {

    }
}