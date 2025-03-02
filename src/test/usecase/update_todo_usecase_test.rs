#[cfg(test)]
mod tests {
    use std::sync::{Arc};
    use crate::application::todo::entity::todo::{Todo, UpdateTodo};
    use crate::test::usecase::mock_todo_repository::MockTodoRepository;
    use crate::application::todo::usecase::update_todo_usecase::UpdateTodoUsecase;
    use crate::common::app_error::AppError;

    #[tokio::test]
    async fn test_update_todo_usecase() {
        let todo_expected = Todo {
            id: 1,
            activity: "activity1".to_string(),
            completed: false,
        };
        let mock_repo = MockTodoRepository::new();

        let usecase = UpdateTodoUsecase::new(Arc::new(mock_repo));
        let update_todo : UpdateTodo = UpdateTodo {
            activity: todo_expected.activity,
            completed: false,
        };
        let todo = usecase.execute(1, update_todo.clone()).await;
        assert!(todo.is_ok());
        assert_eq!(todo.unwrap().activity, update_todo.activity);
    }

    #[tokio::test]
    async fn test_update_todo_notfound_usecase() {
        let mock_repo = MockTodoRepository::new();

        let usecase = UpdateTodoUsecase::new(Arc::new(mock_repo));
        let update_todo : UpdateTodo = UpdateTodo {
            activity: "activity1".to_string(),
            completed: false,
        };
        let todo = usecase.execute(2, update_todo.clone()).await;
        assert!(matches!(todo, Err(AppError::NotFound)));
    }
}