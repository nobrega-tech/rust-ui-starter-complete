#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Review,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub status: ProjectStatus,
}

impl Project {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            status: ProjectStatus::Planning,
        }
    }
}
