#![allow(dead_code)]

use dotenvy::dotenv;
use std::env;

struct ImapClient {
    session: imap::Session<native_tls::TlsStream<std::net::TcpStream>>,
}

impl ImapClient {
    fn new() -> ImapClient {
        let session = mailer();
        ImapClient { session }
    }

    fn logout(mut self) {
        self.session.logout().unwrap();
    }
}

pub fn mailer() -> imap::Session<native_tls::TlsStream<std::net::TcpStream>> {
    dotenvy::dotenv().ok();
    dotenv().expect(".env file not found");
    let user = env::var("EMAIL_USER").expect("EMAIL_USER not defined");
    let password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not defined");
    let imap_host = env::var("IMAP_HOST").expect("IMAP_HOST not defined");
    let imap_port = env::var("IMAP_PORT").expect("IMAP_PORT not defined");
    println!(
        "connecting to imap server: {imap_host}:{imap_port}, with username: {}",
        user.clone()
    );
    let tls = native_tls::TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect(
        (imap_host.clone(), imap_port.parse::<u16>().unwrap()),
        imap_host,
        &tls,
    )
    .unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let imap_session = client.login(user, password).map_err(|e| e.0).unwrap();

    imap_session
}

fn read_mail(client: ImapClient) {
    let mut session = client.session;

    // we want to fetch the first email in the INBOX mailbox
    session.select("INBOX").expect("Can not open INBOX");
    let messages = session.fetch("1", "RFC822").expect("Faild to fetch message");
    let message = messages.iter().next().unwrap();

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    println!("body: {}", body);
    // session.logout().unwrap();
}

fn main() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_mail() {
        let client = ImapClient::new();
        read_mail(client);
    }
}
