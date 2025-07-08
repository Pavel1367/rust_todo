
#[derive(Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub completed: bool,
    pub selected: bool,
    pub created_at: String,
}

impl Todo {
    pub fn from_db(id: i64, text: String, completed: bool, created_at: String) -> Self {
        Todo {
            id,
            text,
            completed,
            selected: false,
            created_at,
        }
    }
}