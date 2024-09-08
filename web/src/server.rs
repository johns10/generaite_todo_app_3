mod app_state;
mod router;
mod server;

use app_state::AppState;
use domain::repository::Repository;
use server::{create_app, start};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize your repository here
    let repository = Repository::new(/* your database connection */);

    let app = create_app(repository)?;
    start(app, "127.0.0.1", 8080).await?;

    Ok(())
}
