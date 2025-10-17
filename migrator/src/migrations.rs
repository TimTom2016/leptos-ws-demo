use sqlx::Sqlite;
use sqlx_migrator::{vec_box, Migration};

mod m0000_setup;
pub(crate) fn migrations() -> Vec<Box<dyn Migration<Sqlite>>> {
    vec_box![m0000_setup::SetupMigration,]
}
