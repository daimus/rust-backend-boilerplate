use async_trait::async_trait;
use diesel::PgConnection;
use crate::application::todo::entity::todo::{CreateTodo, Todo, UpdateTodo};
use crate::application::todo::repository::todo_repository::{TodoRepository};
use diesel::r2d2::{ConnectionManager, Pool};
use crate::infrastructure::data::database::schema::todos::dsl::*;
use diesel::prelude::*;
use crate::infrastructure::data::database::model::todo_model::{CreateTodoModel, TodoModel, TodoModelList, TodoModelOption};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct DatabaseTodoRepository {
    pool: DbPool,
}
impl DatabaseTodoRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl TodoRepository for DatabaseTodoRepository {
    async fn get_todos(&self) -> Vec<Todo> {
        let mut connection = self.pool.get().expect("Failed to get DB connection");
        let result = todos.load::<TodoModel>(&mut connection).unwrap_or_else(|_| vec![]);
        TodoModelList(result).into()
    }

    async fn get_todo(&self, todo_id: i32) -> Option<Todo> {
        let mut connection = self.pool.get().expect("Failed to get DB connection");
        let result = todos.find(todo_id).select(TodoModel::as_select()).first(&mut connection).optional();
        TodoModelOption(result.unwrap()).into()
    }

    async fn create_todo(&self, todo: CreateTodo) -> Todo {
        let mut connection = self.pool.get().expect("Failed to get DB connection");
        let payload : CreateTodoModel = todo.into();
        let result = diesel::insert_into(todos)
            .values(payload)
            .returning(TodoModel::as_returning())
            .get_result(&mut connection)
            .expect("Error saving new todo");
        result.into()
    }

    async fn update_todo(&self, todo_id: i32, todo: UpdateTodo) -> Todo {
        let mut connection = self.pool.get().expect("Failed to get DB connection");
        let result = diesel::update(todos.filter(id.eq(todo_id)))
            .set(
                (
                    activity.eq(todo.activity),
                    completed.eq(todo.completed)
                    )
            )
            .returning(TodoModel::as_returning())
            .get_result(&mut connection)
            .expect("Error saving todo");
        result.into()
    }

    async fn delete_todo(&self, todo_id: i32) {
        let mut connection = self.pool.get().expect("Failed to get DB connection");
        diesel::delete(todos.filter(id.eq(todo_id))).execute(&mut connection).expect("Error delete todo");
    }
}