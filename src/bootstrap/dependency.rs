use std::env;
use std::sync::Arc;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::application::todo::repository::todo_repository::TodoRepository;
use crate::application::todo::usecase::create_todo_usecase::CreateTodosUsecase;
use crate::application::todo::usecase::delete_todo_usecase::DeleteTodoUsecase;
use crate::application::todo::usecase::get_todo_usecase::GetTodoUsecase;
use crate::application::todo::usecase::get_todos_usecase::GetTodosUsecase;
use crate::application::todo::usecase::update_todo_usecase::UpdateTodoUsecase;
use crate::infrastructure::data::database::database_todo_repository::DatabaseTodoRepository;

#[derive(Clone)]
pub struct DependencyContainer {
    pub get_todo_usecase: Arc<GetTodoUsecase>,
    pub get_todos_usecase: Arc<GetTodosUsecase>,
    pub create_todo_usecase: Arc<CreateTodosUsecase>,
    pub update_todo_usecase: Arc<UpdateTodoUsecase>,
    pub delete_todo_usecase: Arc<DeleteTodoUsecase>
}

impl DependencyContainer {
    pub fn new() -> Self {
        let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL not set"));
        let pool: Pool<ConnectionManager<PgConnection>> = Pool::builder().build(manager).expect("Failed to create pool");

        let todo_repository = Arc::new(DatabaseTodoRepository::new(pool.clone())) as Arc<dyn TodoRepository + Send + Sync>;

        let get_todo_usecase = Arc::new(GetTodoUsecase::new(Arc::clone(&todo_repository)));
        let get_todos_usecase = Arc::new(GetTodosUsecase::new(Arc::clone(&todo_repository)));
        let create_todo_usecase = Arc::new(CreateTodosUsecase::new(Arc::clone(&todo_repository)));
        let update_todo_usecase = Arc::new(UpdateTodoUsecase::new(Arc::clone(&todo_repository)));
        let delete_todo_usecase = Arc::new(DeleteTodoUsecase::new(Arc::clone(&todo_repository)));

        Self {
            get_todo_usecase,
            get_todos_usecase,
            create_todo_usecase,
            update_todo_usecase,
            delete_todo_usecase
        }
    }
}