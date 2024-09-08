mod db;

use db::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgres://username:password@localhost/database_name";
    let db = Database::new(database_url).await?;
    
    println!("Database connection established!");
    
    Ok(())
}
