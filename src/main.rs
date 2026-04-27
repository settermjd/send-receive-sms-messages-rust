use axum::{Form, Router, routing::post};
use rand::prelude::*;
use serde::Deserialize;

/// TwilioMessagingRequest is used to deserialise POST requests from Twilio
/// Requests will contain a single element named "Body", which contains the
/// body of an SMS sent by the user to a Twilio phone number.
#[derive(Deserialize)]
struct TwilioMessagingRequest {
    #[serde(rename = "Body")]
    body: String,
}

#[tokio::main]
async fn main() {
    // Set up the application's routing table with two routes
    // One that will not send a reply SMS, and one that will send a reply SMS.
    let app = Router::new()
        .route("/receive/no-response", post(no_response))
        .route("/receive/with-response", post(with_response));

    // Run the application with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    // Listen for incoming requests
    axum::serve(listener, app).await.unwrap();
}

/// This function handles requests to the "/receive/no-response" route.
///
///
/// The function's response will contain TwiML that provides no further instructions to Twilio.
/// In addition the response's status code will be an HTTP 200 OK, and the response will have
/// the Content-Type header set to "application/xml; charset=utf-8".
async fn no_response() -> &'static str {
    r#"<?xml version="1.0" encoding="UTF-8"?><Response></Response>"#
}

/// This function handles requests to the "/receive/with-response" route.
///
/// If the request's form data contains an element named "Body" with the value "never gonna",
/// the function's response will contain TwiML that instructs Twilio to send a reply SMS to
/// the sender of the original SMS with a line from Rick Astley's hit "Never Gonna Give You Up".
/// Otherwise it sends the same, stock line from the same song.
///
/// In addition the response's status code will be an HTTP 200 OK, and the response will have
/// the Content-Type header set to "application/xml; charset=utf-8".
async fn with_response(Form(request): Form<TwilioMessagingRequest>) -> String {
    let default_option: &str = "I just wanna tell you how I'm feeling - Gotta make you understand";
    let options: Vec<&str> = vec![
        "give you up",
        "let you down",
        "make you cry",
        "run around and desert you",
        "say goodbye",
        "tell a lie, and hurt you",
    ];

    if request.body.as_str().to_lowercase() == "never gonna" {
        let mut rng = rand::rng();
        let index = rng.random_range(0..options.len());
        let message = options[index];
        return format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><Response><Message>{message}</Message></Response>"#
        );
    }

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Response><Message>{default_option}</Message></Response>"#
    )
}
