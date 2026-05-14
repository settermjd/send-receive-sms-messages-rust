<!-- markdownlint-disable MD013 -->

# Send & receive SMS messages with Rust

Learn how to send and receive SMS with Twilio and Rust with this repository.
In just a few lines of code, you can see your phone light up sending and receiving SMS with Twilio and Rust.

## Prerequisites

To run the app locally, you need the following:

- Rust and Cargo
- [ngrok][ngrok] and a free ngrok account
- A [Twilio account][twilio_signup] with an active phone number that can send SMS
- Some command-line/terminal experience would be helpful, but it's not necessary

## Quickstart

First things first, clone or download this repository.
In the repository, you'll see two folders: _send_sms_ and _receive_sms_.
_send_sms_ contains just enough code to send an SMS to a recipient from your Twilio phone number.
_receive_sms_ contains a small, web application (using the [Axum] framework) that can reply to an SMS received by your Twilio phone number.

### Send an SMS

Before you can send an SMS, you need to complete the following

In the _send_sms_ directory:

1. Rename the `.env.example` file to `.env`
1. Go to the [Twilio Console][twilio_console] and find your **Account SID**, **Auth Token**, and Twilio phone number.
1. Copy and paste those values into the placeholders in the `.env` file `TWILIO_ACCOUNT_SID`, `TWILIO_AUTH_TOKEN`, and `SENDER`, respectively.
1. Set your phone number, in [E.164 format][e164_format] as the value of `RECIPIENT` in _.env_
   Save the file.

Then, run the following command to send an SMS.

```bash
cargo run
```

### Receive an SMS

Before you can receive an SMS, you need to complete a few further steps.

1. Start your ngrok server:

   ```bash
   ngrok http 8080
   ```

1. Go to the [Active numbers][active_numbers] page in the Twilio Console.
1. Click your Twilio phone number.
1. Go to the **Configure** tab and find the **Messaging Configuration** section.
1. In the **A call comes in** row, select the **Webhook** option.
1. Paste your ngrok **Forwarding** URL in the **URL** field followed by "/receive/".
   For example, if your ngrok console shows Forwarding "<https://1aaa-123-45-678-910.ngrok-free.app>", enter "<https://1aaa-123-45-678-910.ngrok-free.app/receive/>".
   - To receive an SMS **without** responding to it, append "no-response" to the URL
   - To receive an SMS and respond to it, append "with-response" to the URL
1. Click **Save configuration**.
1. Start the Rust web app

   ```bash
   cargo run
   ```

1. With both the Rust app and ngrok running, send an SMS to your Twilio phone number, containing whatever message you like.
   If you want a response, try sending "never gonna" as the message.

[active_numbers]: https://console.twilio.com/us1/develop/phone-numbers/manage/incoming
[axum]: https://docs.rs/axum/latest/axum/
[e164_format]: https://www.twilio.com/docs/glossary/what-e164
[ngrok]: https://ngrok.com/
[twilio_console]: https://console.twilio.com
[twilio_signup]: https://www.twilio.com/try-twilio

<!-- markdownlint-enable MD013 -->
