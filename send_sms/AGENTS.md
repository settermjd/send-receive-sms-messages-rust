<!-- markdownlint-disable MD013 -->

# AGENTS Instructions

## Project Overview

This project demonstrates how to send and reply to incoming SMS with Twilio and Rust.

## Prerequisites

- Rust version 1.85.1 or above
- [ngrok][ngrok] and a free ngrok account
- A [Twilio account][twilio_signup] with an active phone number that can send SMS
- Some command-line/terminal experience would be helpful, but it's not necessary

## Environment Set up

- Clone or download this repository

### For Replying to an SMS

- Start ngrok listening on port 8080:

  ```bash
  ngrok http 8080
  ```

1. Go to the [Active numbers][active_numbers] page in the Twilio Console.
1. Click your Twilio phone number.
1. Go to the **Configure** tab and find the **Messaging Configuration** section
1. In the **A call comes in** row, select the **Webhook** option.
1. Paste your ngrok **Forwarding** URL in the **URL** field followed by "/receive/"; then:
   - Append "no-response" to the URL to receive an SMS **without** responding
   - Append "with-response" to the URL to receive an SMS and respond
1. Click **Save configuration**.
1. Start the Rust web app:

   ```bash
   cargo run
   ```

## Code Style Guidelines

The code style for this project follows the [Rust Style Guide][rust-style-guide].

## Commit Messages and Pull Requests

- Follow [the Chris Beams style of commit messages][chris-beams-commit-message].
- Every pull request should answer:
  - What changed?
  - Why?
  - What are the breaking changes?
  - What is the server PR (if the change requires a coordinated server update)?
- Comments should be complete sentences and end with a period.

## Review Checklist

- Add new tests for any new feature or bug fix.
- Update the documentation when necessary.

For more details see [CONTRIBUTING.md](../CONTRIBUTING.md) in the repository root.

[active_numbers]: https://console.twilio.com/us1/develop/phone-numbers/manage/incoming
[chris-beams-commit-message]: http://chris.beams.io/posts/git-commit/
[ngrok]: https://ngrok.com/
[rust-style-guide]: https://doc.rust-lang.org/style-guide/
[twilio_signup]: https://www.twilio.com/try-twilio

<!-- markdownlint-enable MD013 -->
