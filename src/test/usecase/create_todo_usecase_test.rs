#[cfg(test)]
mod tests {
    use std::sync::{Arc};
    use crate::application::todo::entity::todo::{CreateTodo, Todo};
    use crate::test::usecase::mock_todo_repository::MockTodoRepository;
    use crate::application::todo::usecase::create_todo_usecase::CreateTodosUsecase;
    #[tokio::test]
    async fn test_create_todo_usecase() {
        let todo_expected = Todo {
            id: 1,
            activity: "activity1".to_string(),
            completed: false,
        };
        let mock_repo = MockTodoRepository::new();
        let usecase = CreateTodosUsecase::new(Arc::new(mock_repo));
        let create_todo : CreateTodo = CreateTodo {
            activity: todo_expected.activity
        };
        let todo = usecase.execute(create_todo.clone()).await;
        assert_eq!(todo.activity, create_todo.activity);
    }
}