use failure::Error;
use reqwest::{Client, StatusCode};
use rustlio::twilio::{ErrorResponse, messaging::MessageResource};

fn handle_error(body: String) {
    let error_response: ErrorResponse =
        serde_json::from_str(&body).expect("Unable to deserialise JSON error response.");
    println!(
        "SMS was not able to be sent because: {:?}.",
        error_response.message
    );
}

fn handle_success(body: String) {
    let sms_response: MessageResource =
        serde_json::from_str(&body).expect("Unable to deserialise JSON success response.");
    println!("Your SMS with the body \"{:?}\".", sms_response.body);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    let recipient = dotenvy::var("RECIPIENT_PHONE_NUMBER")?;
    let sender = dotenvy::var("TWILIO_PHONE_NUMBER")?;
    let account_sid = dotenvy::var("TWILIO_ACCOUNT_SID")?;
    let auth_token = dotenvy::var("TWILIO_AUTH_TOKEN")?;

    let request_params = [
        ("To", recipient),
        ("From", sender),
        ("Body", "G'day from Rust and Twilio".to_string()),
    ];
    let client = Client::new();
    let response = client
        .post(format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            account_sid,
        ))
        .form(&request_params)
        .basic_auth(account_sid, Some(auth_token))
        .send()
        .await?;

    let status = response.status();
    let body = match response.text().await {
        Ok(result) => result,
        Err(error) => panic!(
            "Problem extracting the JSON body content. Reason: {:?}",
            error
        ),
    };

    match status {
        StatusCode::BAD_REQUEST => handle_error(body),
        StatusCode::OK => handle_success(body),
        _ => println!("Received status code: {}", status),
    }

    Ok(())
}
