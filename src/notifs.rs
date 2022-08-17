use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use std::env;

pub async fn send_email(email_body: String) -> Result<&'static str, Box<dyn std::error::Error>> {
    let email_username: String =
        env::var("EMAIL_USERNAME").expect("$EMAIL_USERNAME env var is not set");
    let email_password: String =
        env::var("EMAIL_PASSWORD").expect("$EMAIL_PASSWORD env var is not set");
    let email_smtp_host: String =
        env::var("EMAIL_SMTP_HOST").expect("$EMAIL_SMTP_HOST env var is not set");
    let email_recipient: String =
        env::var("EMAIL_RECIPIENT").expect("$EMAIL_RECIPIENT env var is not set");

    let smtp_credentials = Credentials::new(email_username, email_password);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&email_smtp_host)?
        .credentials(smtp_credentials)
        .build();

    let from: &str = "DoomAlerts Bot <doom@alerts.rs>";
    let to: &str = &email_recipient;
    // TODO: extract date
    let subject: &str = "KPLC Scheduled interruptions for _tomorrow_";

    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(email_body)?;

    mailer.send(email).await?;

    Ok("SUCCESS: Email sent!")
}

// TODO: SMS, Telegram
