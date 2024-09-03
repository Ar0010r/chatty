use domain::{
    dto::company::{CompanyBody, CompanyProps},
    repositories::manager::ManagerEraser,
    source::repository::delete::DeletesData,
    Factory,
};
use fake::{Fake, Faker};

use crate::{actions::company, test::prepare_admin};

#[tokio_shared_rt::test]
pub async fn check_crud() {
    let admin = prepare_admin().await;
    let mut request = Factory::<CompanyBody>::create_one();
    request.manager_id = admin.id;
    request.domain = format!("http://{}.com", request.domain);

    dbg!(&request);

    let create_result = company::create(request, &admin).await;

    dbg!(&create_result);

    assert!(create_result.is_ok());

    let company = create_result.unwrap();
    let show_result = company::show(company.id, &admin).await;

    assert!(show_result.is_ok());

    let update_request = CompanyProps {
        name: Some(Faker.fake()),
        ..Default::default()
    };

    let update_result = company::update(company.id, update_request.clone(), &admin).await;
    let updated_company = update_result.clone().unwrap();

    assert!(update_result.is_ok());
    assert_eq!(updated_company.name.clone(), update_request.name.unwrap());

    let delete_result = company::delete(company.id, &admin).await;

    assert!(delete_result.is_ok());
    assert!(delete_result.unwrap());

    let admin_deleted = ManagerEraser::by_id(admin.id, None, None).await;

    assert_eq!(admin_deleted.unwrap_or_default(), 1);
}
