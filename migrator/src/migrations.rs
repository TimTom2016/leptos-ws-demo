pub mod m0000_setup;
pub mod m0001_chat;
pub mod m0002_join_code;

use sqlx_migrator::{Migration, vec_box};

pub fn migrations() -> Vec<Box<dyn Migration<sqlx::Sqlite>>> {
    vec_box![
        m0000_setup::SetupMigration,
        m0001_chat::ChatMigration,
        m0002_join_code::JoinCodeMigration,
    ]
}
