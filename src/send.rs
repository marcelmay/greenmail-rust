#![allow(dead_code)]

use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use dotenvy::dotenv;
use std::env;
pub struct Mail {
    from: Mailbox,
    to: Mailbox,
    subject: String,
    body: String,
}

pub fn mailer() -> SmtpTransport {
    dotenvy::dotenv().ok();
    dotenv().expect(".env file not found");
    let smtp_user = env::var("EMAIL_USER").expect("USER not defined");
    let smtp_password = env::var("EMAIL_PASSWORD").expect("PASSWORD not defined");
    let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST not defined");
    let smtp_port = env::var("SMTP_PORT").expect("SMTP_PORT not defined");
    println!(
        "connecting to smtp server: {smtp_host}, with username: {}",
        smtp_user.clone()
    );

    let creds = Credentials::new(smtp_user, smtp_password);

    
    SmtpTransport::relay(&smtp_host)
        .unwrap()
        .port(smtp_port.parse::<u16>().unwrap())
        .credentials(creds)
        .build()
}

pub fn compose(mail: &Mail) -> Message {
    

    Message::builder()
        .from(mail.from.clone())
        .reply_to(mail.from.clone())
        .to(mail.to.clone())
        .subject(mail.subject.clone())
        .header(ContentType::TEXT_PLAIN)
        .body(mail.body.clone())
        .unwrap()
}

pub fn send_mail(mailer: &SmtpTransport) {
    let mail = Mail {
        from: Mailbox {
            name: None,
            email: "from@localhost".parse().unwrap(),
        },
        to: Mailbox {
            name: None,
            email: "to@localhost".parse().unwrap(),
        },
        subject: "foo subject".to_string(),
        body: "bar body".to_string(),
    };

    let email = compose(&mail);

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_mail() {
        let mailer: SmtpTransport = mailer();
        send_mail(&mailer);
    }
}
