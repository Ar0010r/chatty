use crate::service::manager::ManagerService;
use domain::{dto::manager::Manager, entities::manager as mngr, Factory};
use system::{config, database::postgres::connection};

mod company;
mod lead;
mod manager;

async fn prepare_admin() -> Manager {
    config::setup();
    let _ = connection::assure().await;

    let mut admin = Factory::<mngr::Model>::create_one();
    admin.role = mngr::Role::Admin;
    let admin = ManagerService::create(admin, None).await.unwrap();

    Manager::from_model(admin)
}
