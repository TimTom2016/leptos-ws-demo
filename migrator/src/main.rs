#![expect(unused_crate_dependencies)]
//! Example crate for postgres
use sqlx::migrate::MigrateDatabase as _;
use sqlx::Sqlite;
use sqlx_migrator::cli::MigrationCommand;
use sqlx_migrator::migrator::{Info, Migrator};

mod migrations;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let uri = std::env::var("DATABASE_URL").unwrap();
    if !Sqlite::database_exists(&uri).await.unwrap_or(false) {
        println!("Creating database {}", uri);
        match Sqlite::create_database(&uri).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let pool = sqlx::Pool::<Sqlite>::connect(&uri).await.unwrap();
    let mut migrator = Migrator::default();
    migrator.add_migrations(migrations::migrations()).unwrap();
    // There are two way to run migration. Either you can create cli as shown below
    let mut conn = pool.acquire().await.unwrap();
    MigrationCommand::parse_and_run(&mut *conn, Box::new(migrator))
        .await
        .unwrap();
    // Or you can directly use migrator run function instead of creating
    // cli
    // migrator
    //     .run(&mut *conn, sqlx_migrator::migrator::Plan::apply_all())
    //     .await
    //     .unwrap();
    conn.close().await.unwrap();
}
