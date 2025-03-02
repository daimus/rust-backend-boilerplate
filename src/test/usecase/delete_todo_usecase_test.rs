#[cfg(test)]
mod tests {
    use std::sync::{Arc};
    use crate::test::usecase::mock_todo_repository::MockTodoRepository;
    use crate::application::todo::usecase::delete_todo_usecase::DeleteTodoUsecase;
    #[tokio::test]
    async fn test_delete_todo_usecase() {
        let mock_repo = MockTodoRepository::new();
        let usecase = DeleteTodoUsecase::new(Arc::new(mock_repo));
        usecase.execute(1).await;
        assert!(true);
    }
}