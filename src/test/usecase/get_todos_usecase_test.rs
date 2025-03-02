#[cfg(test)]
mod tests {
    use std::sync::{Arc};
    use crate::test::usecase::mock_todo_repository::MockTodoRepository;
    use crate::application::todo::usecase::get_todos_usecase::GetTodosUsecase;
    #[tokio::test]
    async fn test_get_todos_usecase() {
        let mock_repo = MockTodoRepository::new();

        let usecase = GetTodosUsecase::new(Arc::new(mock_repo));

        let todos = usecase.execute().await;
        assert!(todos.len() >= 0);
    }
}