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

#[derive(Debug)]
pub struct Notification<'a> {
    pub tx_id: String,
    pub coin: &'a str,
    pub wallet: &'a str,
    pub amount: u64,
}

#[tracing::instrument(level = "info")]
pub async fn dispatch(notification: &Notification<'_>) -> bool {
    info!("Dispatching email notification");

    let response = match HTTP_CLIENT
        .post("https://api.brevo.com/v3/smtp/email")
        .header("api-key", &CONFIG.api_key)
        .json(&build_email_payload(&notification))
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

fn build_email_payload(notification: &Notification<'_>) -> EmailPayload {
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
            Wallet Address: {}
            Blockchain: ERG
            Coin: {}
            Amount: {}
            Transaction Hash: {}
            "#,
            notification.wallet,
            notification.coin,
            to_decimal(notification.amount, 9),
            notification.tx_id
        )
        .trim()
        .to_string(),
    };
    payload
}

fn to_decimal(value: u64, decimals: i32) -> f64 {
    let factor = 10f64.powi(decimals);
    value as f64 / factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_decimal_zero() {
        assert_eq!(to_decimal(0, 9), 0.0);
    }

    #[test]
    fn test_to_decimal_no_decimals() {
        assert_eq!(to_decimal(12345, 0), 12345.0);
    }

    #[test]
    fn test_to_decimal_basic() {
        assert_eq!(to_decimal(1000000000, 9), 1.0);
    }

    #[test]
    fn test_to_decimal_fractional() {
        let result = to_decimal(123456789, 6);
        assert!((result - 123.456789).abs() < 1e-6);
    }

    #[test]
    fn test_to_decimal_large_decimals() {
        let result = to_decimal(1, 12);
        assert!((result - 0.000000000001).abs() < 1e-15);
    }
}
