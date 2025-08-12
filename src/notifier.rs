use std::env;

use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::Value;
use tracing::{error, info};

use crate::HTTP_CLIENT;

struct NotificationServiceConfig {
    sender_email: String,
    receiver_email: String,
    api_key: String,
}

static CONFIG: Lazy<NotificationServiceConfig> = Lazy::new(|| {
    let sender_email =
        env::var("ERGO_NOTIFY_SENDER_EMAIL").expect("ERGO_NOTIFY_SENDER_EMAIL must be set");
    let receiver_email =
        env::var("ERGO_NOTIFY_RECEIVER_EMAIL").expect("ERGO_NOTIFY_RECEIVER_EMAIL must be set");
    let api_key = env::var("ERGO_EMAIL_API_KEY").expect("ERGO_EMAIL_API_KEY must be set");

    NotificationServiceConfig {
        sender_email,
        receiver_email,
        api_key,
    }
});

#[tracing::instrument(skip(tx_ids))]
pub async fn dispatch(tx_ids: &[&str]) -> bool {
    info!("Dispatching email notification");

    let response = match HTTP_CLIENT
        .post("https://api.brevo.com/v3/smtp/email")
        .header("api-key", &CONFIG.api_key)
        .json(&build_email_payload(tx_ids))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            error!(error = ?e, "Failed to send email");
            return false;
        }
    };

    if response.status().is_success() {
        true
    } else {
        let body: Value = match response.json().await {
            Ok(val) => val,
            Err(e) => {
                error!(error = ?e, "Failed to parse response body");
                Value::String("Error parsing response body".to_string())
            }
        };

        error!(response_body = ?body, "Failed to send email");
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EmailPayload {
    sender: Sender,
    to: Vec<Recipient>,
    subject: String,
    text_content: String,
}

#[derive(Debug, Serialize)]
struct Sender {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct Recipient {
    email: String,
}

fn build_email_payload(tx_ids: &[&str]) -> EmailPayload {
    let payload = EmailPayload {
        sender: Sender {
            name: "Ergo Notifier".into(),
            email: CONFIG.sender_email.clone(),
        },
        to: vec![Recipient {
            email: CONFIG.receiver_email.clone(),
        }],
        subject: "Ergo Transactions Notification".into(),
        text_content: format!(
            r#"
            **Confirmed Transactions:**
            {}"#,
            tx_ids.join("\n")
        )
        .trim()
        .to_string(),
    };
    payload
}
