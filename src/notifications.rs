use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::env;

pub async fn send_email(
    interruption_date: String,
    tweet_link: String,
) -> Result<&'static str, Box<dyn std::error::Error>> {
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
    let subject: String = format!("KPLC Scheduled Interruptions for {interruption_date}");
    let (body_text, body_html) = build_email_body(tweet_link);

    let email: Message = Message::builder()
        .from(from.parse()?)
        .to(email_recipient.as_str().parse()?)
        .subject(&subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(body_text),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(body_html),
                ),
        )?;

    mailer.send(email).await?;

    Ok("SUCCESS: Email sent!")
}

/// This function appends the tweet link to the email body and returns a tuple of the text and html versions.
fn build_email_body(tweet_link: String) -> (String, String) {
    let html = format!(
        r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Hello from DoomAlerts!</title>
    </head>
    <body>
        <div style="display: flex; flex-direction: column; align-items: center;">
            <h4 style="font-family: Arial, Helvetica, sans-serif;">Hello from DoomAlerts!</h4>
        </div>
        <div>
            <p>An area on your DoomAlerts watchlist may be affected by a scheduled power interruption. Click here to view the full details on Twitter: {tweet_link}</p>
        </div>
    </body>
    </html>"#
    );

    let text = format!(
        r#"Hello from DoomAlerts!
    
        An area on your DoomAlerts watchlist may be affected by a scheduled power interruption. Click here to view the full details on Twitter: {tweet_link}"#
    );

    (text, html)
}

// TODO: SMS, Telegram
