use domain::repository::Repository;
use std::sync::Arc;
use tera::Tera;

pub struct AppState {
    pub repository: Arc<Repository>,
    pub templates: Tera,
}

impl AppState {
    pub fn new(repository: Repository, templates: Tera) -> Self {
        Self {
            repository: Arc::new(repository),
            templates,
        }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            repository: Arc::clone(&self.repository),
            templates: self.templates.clone(),
        }
    }
}
