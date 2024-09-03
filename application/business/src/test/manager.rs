use domain::{
    dto::manager::{Manager, ManagerBody, ManagerProps},
    entities::manager,
    source::repository::delete::{DataEraser, DeletesData},
    Factory,
};
use system::{exception::model::Exception, various::auth::issue_token};

use crate::{actions, test::prepare_admin, various::auth::Credentials};

#[tokio_shared_rt::test]
async fn check_crud() {
    let admin = prepare_admin().await;
    let ts = chrono::Utc::now().timestamp();

    let creds = Credentials {
        login: ts.to_string(),
        password: format!("test_{}", ts),
    };

    assert!(admin.id > 0);

    let show_result = actions::manager::show(admin.id, &admin).await;
    dbg!(&show_result);

    assert!(show_result.is_ok());

    let register_result = register(&admin, &creds).await;

    dbg!(&register_result);

    assert!(register_result.is_ok());

    let login_result = issue_token(creds).await;

    dbg!(&login_result);

    assert!(login_result.is_ok());

    let model = register_result.unwrap();
    let request = ManagerProps {
        login: Some(chrono::Utc::now().timestamp().to_string()),
        ..Default::default()
    };

    let update_result = actions::manager::update(model.id, request.clone(), &admin).await;
    let updated_model = update_result.clone().unwrap();

    dbg!(&update_result);

    assert!(update_result.is_ok());
    assert_eq!(updated_model.login.clone(), request.login.unwrap());

    let ids = vec![admin.id, updated_model.id];
    let delete = DataEraser::<manager::Entity>::by_ids(ids.clone(), None, None).await;

    dbg!(&delete);

    assert_eq!(delete.unwrap_or_default(), ids.len() as u64);
}

async fn register(admin: &Manager, creds: &Credentials) -> Result<manager::Model, Exception> {
    let mut manager = Factory::<ManagerBody>::create_one();
    manager.password = creds.password.clone();
    manager.login = creds.login.clone();

    actions::manager::create(manager, admin).await
}
