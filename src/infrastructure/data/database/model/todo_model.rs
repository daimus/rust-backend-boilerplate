use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::application::todo::entity::todo::{CreateTodo, Todo, UpdateTodo};

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Selectable, Deserialize)]
#[diesel(table_name = crate::infrastructure::data::database::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoModel {
    pub id: i32,
    pub activity: String,
    pub completed: bool,
}
impl From<TodoModel> for Todo {
    fn from(db_model: TodoModel) -> Self {
        Todo {
            id: db_model.id,
            activity: db_model.activity,
            completed: db_model.completed,
        }
    }
}
impl From<Todo> for TodoModel {
    fn from(domain_model: Todo) -> Self {
        TodoModel {
            id: domain_model.id,
            activity: domain_model.activity,
            completed: domain_model.completed,
        }
    }
}
pub struct TodoModelOption(pub Option<TodoModel>);
impl From<TodoModelOption> for Option<Todo> {
    fn from(opt_model: TodoModelOption) -> Self {
        opt_model.0.map(Todo::from)
    }
}
pub struct TodoModelList(pub Vec<TodoModel>);
impl From<TodoModelList> for Vec<Todo> {
    fn from(todo_models: TodoModelList) -> Self {
        todo_models.0.into_iter().map(Todo::from).collect()
    }
}
#[derive(Debug, Clone, Serialize, Queryable, Insertable, Selectable, Deserialize)]
#[diesel(table_name = crate::infrastructure::data::database::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateTodoModel {
    pub activity: String,
}
impl From<CreateTodo> for CreateTodoModel {
    fn from(todo: CreateTodo) -> Self {
        Self {
            activity: todo.activity
        }
    }
}
#[derive(Debug, Clone, Serialize, Queryable, Insertable, Selectable, Deserialize)]
#[diesel(table_name = crate::infrastructure::data::database::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateTodoModel {
    pub activity: String,
    pub completed: bool,
}
impl From<UpdateTodo> for UpdateTodoModel {
    fn from(todo: UpdateTodo) -> Self {
        Self {
            activity: todo.activity,
            completed: todo.completed
        }
    }
}