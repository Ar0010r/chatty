use domain::{
    dto::lead::{LeadBody, LeadProps},
    entities::manager,
    source::repository::delete::{DataEraser, DeletesData},
    Factory,
};
use fake::{faker::internet::en::SafeEmail, Fake, Faker};

use crate::{actions::lead, test::prepare_admin};

#[tokio_shared_rt::test]
pub async fn check_crud() {
    let admin = prepare_admin().await;
    let mut request = Factory::<LeadBody>::create_one();
    request.hr_id = Some(admin.id);
    request.emails = vec![SafeEmail().fake()];
    request.company_id = None;
    request.hr_company_id = None;

    dbg!(&request);

    let create_result = lead::create(request, &admin).await;

    dbg!(&create_result);

    assert!(create_result.is_ok());

    let lead = create_result.unwrap();
    let show_result = lead::show(lead.id, &admin).await;

    assert!(show_result.is_ok());

    let update_request = LeadProps {
        first_name: Some(Faker.fake()),
        ..Default::default()
    };

    let update_result = lead::update(lead.id, update_request.clone(), &admin).await;
    let updated_lead = update_result.clone().unwrap();

    assert!(update_result.is_ok());
    assert_eq!(
        updated_lead.first_name.clone(),
        update_request.first_name.unwrap()
    );

    let delete_result = lead::delete(lead.id, &admin).await;

    assert!(delete_result.is_ok());
    assert!(delete_result.unwrap());

    let admin_deleted = DataEraser::<manager::Entity>::by_id(admin.id, None, None).await;

    assert_eq!(admin_deleted.unwrap_or_default(), 1);
}
