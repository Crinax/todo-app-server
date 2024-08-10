pub struct UpdateTaskTitleCommand {
    id: String,
    title: String,
}

impl UpdateTaskTitleCommand {
    pub fn new(id: String, title: String) -> Self {
        Self { id, title }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
