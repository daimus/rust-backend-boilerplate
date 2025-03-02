#[derive(Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub activity: String,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct CreateTodo {
    pub activity: String,
}

#[derive(Debug, Clone)]
pub struct UpdateTodo {
    pub activity: String,
    pub completed: bool,
}