#[cfg(test)]
mod tests {
    use std::sync::{Arc};
    use crate::test::usecase::mock_todo_repository::MockTodoRepository;
    use crate::application::todo::usecase::get_todo_usecase::GetTodoUsecase;
    #[tokio::test]
    async fn test_get_todo_usecase() {
        let mock_repo = MockTodoRepository::new();
        let usecase = GetTodoUsecase::new(Arc::new(mock_repo));
        let todo = usecase.execute(1).await.unwrap();
        assert_eq!(todo.activity, "activity1");
    }
    #[tokio::test]
    async fn test_get_todo_usecase_notfound() {
        let mock_repo = MockTodoRepository::new();
        let usecase = GetTodoUsecase::new(Arc::new(mock_repo));
        let todo = usecase.execute(2).await;
        assert!(todo.is_none());
    }
}