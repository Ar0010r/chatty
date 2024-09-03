use std::{collections::HashMap, thread};

use strfmt::strfmt;

use rand::Rng;

pub fn get_greeting(lead: &lead::Model, reply_to: Option<Message>) -> Envelope {
    let mut vars = HashMap::new();
    let first_name = lead.first_name.clone().unwrap_or_default();

    let subj_name = match first_name.as_str() {
        "" => "you".to_string(),
        _ => first_name.clone(),
    };

    let body_name = match first_name.as_str() {
        "" => "applicant".to_string(),
        _ => first_name,
    };

    // let body_name = match subj_name.as_str() {
    //     "you" => "applicant".to_string(),
    //     n => n.to_string()
    // };

    vars.insert("subj_name".to_string(), subj_name.as_str());
    vars.insert("body_name".to_string(), body_name.as_str());
    let subject_template = SUBJECTS[rand::thread_rng().gen_range(0..SUBJECTS.len())];
    let body_template = BODIES[rand::thread_rng().gen_range(0..BODIES.len())];

    let subject = strfmt(subject_template, &vars).unwrap();
    let body = strfmt(body_template, &vars).unwrap();

    let mut greeting = Envelope::make(
        lead.emails[0].as_str(),
        &subject,
        &body,
        &vec!["/app/smart_trucks_zhs.pdf".to_string()],
        reply_to,
    );
    greeting.subject = subject;

    greeting
}

pub async fn greet(leads: Vec<lead::Model>, span: i16) -> Result<(), Exception> {
    let service = GmailService::build("support@zhsrecruitment.com".to_string()).await;
    //let service = GmailService::build("noreply@code-confirm.com".to_string()).await;
    let sleep = match span {
        0 => std::time::Duration::from_secs(360),
        _ => std::time::Duration::from_secs(span as u64),
    };

    for lead in leads.iter() {
        let last_msg = service.find_last_recieved(&lead.emails[0]).await?;
        let message = get_greeting(lead, last_msg);
        let result = service.dispatch(message).await;

        if result.is_ok() {
            LeadLogService::log_lead(lead, Action::Spam).await?;
        } else {
            println!("{:?}", result.err());
            panic!("Error sending message to {}", lead.emails[0]);
        }

        thread::sleep(sleep);
    }

    Ok(())
}

pub async fn sendout(filter: LeadFilter, wave: i16) -> bool {
    let leads = LeadReader::sendout_list(filter, wave).await;

    println!("{:?}", leads);
    greet(leads, i16::default()).await.is_ok()
}

pub const SUBJECTS: [&str; 6] = [
    SUBJECT, SUBJECT_1, SUBJECT_2, SUBJECT_3, SUBJECT_4, SUBJECT_5,
];
pub const SUBJECT: &str = "Job offer 4000 USD for {subj_name}";
pub const SUBJECT_1: &str = "4000 USD job offer for {subj_name}";
pub const SUBJECT_2: &str = "$4000 job for {subj_name}";
pub const SUBJECT_3: &str = "$4000 job opportunity for {subj_name}";
pub const SUBJECT_4: &str = "$4000 career opportunity for {subj_name}";
pub const SUBJECT_5: &str = "New $4000 job offer for {subj_name}";

pub const BODIES: [&str; 6] = [BODY, BODY_1, BODY_2, BODY_3, BODY_4, BODY_5];

pub const BODY: &str = "Dear {body_name},

I'm HR manager at ZHS Recruitment. We have an offer from Smart Truck Delivery company.

They have only one vacant position, and you can apply for it right now.
I have attached the file with its description to this message.
Please read it carefully and write to me if you have any questions about the position or want to apply.

Waiting for your response today!
_________________________________
Best regards,
HR manager at ZHS Recruitment
";

pub const BODY_1: &str = "Dear {body_name},

I'm HR manager at ZHS Recruitment. We have an offer from Smart Truck Delivery company.

They have only one vacant position, and you can apply for it right now.
I have attached the file with its description to this message.
Please read it carefully and write to me if you have any questions about the position or want to apply.

Waiting for your response today!
_________________________________
Best regards,
HR manager at ZHS Recruitment
";

pub const BODY_2: &str = "Dear {body_name},

As the HR manager at ZHS Recruitment, I wanted to inform you of an exciting opportunity with Smart Truck Delivery.

They're hiring for a single position, and you can apply immediately. I've included the job details in the attached file. Please review it and get in touch if you have any questions or wish to proceed with your application.

I'm awaiting your response today!
_________________________________
Best regards,
HR manager at ZHS Recruitment
";

pub const BODY_3: &str = "Dear {body_name},

I'm the HR manager here at ZHS Recruitment, and I'm reaching out about a job opening at Smart Truck Delivery.

There's one spot available, and I wanted to give you the chance to apply right away. I've attached the job description for your review. If you have any questions or are ready to apply, please let me know.

I look forward to your reply today!
_________________________________
Best regards,
HR manager at ZHS Recruitment
";

pub const BODY_4: &str = "Dear {body_name},

As the HR manager at ZHS Recruitment, I'm pleased to inform you of an offer from Smart Truck Delivery.

They have one vacancy, and you can submit your application now. The job description is attached to this message. Please go through it and feel free to contact me if you have any queries or are interested in applying.

I'm eager to hear back from you today!
_________________________________
Best regards,
HR manager at ZHS Recruitment
";

pub const BODY_5: &str = "Dear {body_name},

I'm reaching out as the HR manager at ZHS Recruitment to let you know about an opportunity with Smart Truck Delivery.

There's only one position available, and you can apply for it right now. I've attached the detailed job description for your review. Please read through it and contact me if you have any questions or would like to apply.

Looking forward to your response today!
_________________________________
Best regards,
HR manager at ZHS Recruitment
";
