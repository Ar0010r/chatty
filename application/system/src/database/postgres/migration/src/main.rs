use migration::Migrator;
use sea_orm_migration::prelude::*;
use system::database::postgres;

#[async_std::main]
async fn main() {
    let _ = postgres::connection::assure().await;
    let connection = postgres::connection::get();

    let _ = Migrator::up(connection, None).await;
}
