use std::sync::Arc;
use domain::repository::Repository;
use tera::Tera;

mod router;
mod server;

#[derive(Clone)]
pub struct AppState {
    repository: Arc<Repository>,
    templates: Arc<Tera>,
}

impl AppState {
    pub fn new(repository: Repository, templates: Tera) -> Self {
        Self {
            repository: Arc::new(repository),
            templates: Arc::new(templates),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize your repository here
    let repository = Repository::new(/* your database connection */);

    let app = server::create_app(repository)?;

    println!("Server starting on http://127.0.0.1:3000");
    server::start(app, "127.0.0.1", 3000).await?;

    Ok(())
}
