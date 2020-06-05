[![Build Status](https://github.com/jewish-interactive/ji-cloud-api/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/jewish-interactive/ji-cloud-api/actions)

# Endpoints

release: https://api.jicloud.org
sandbox: https://sandbox.api.jicloud.org

# Dev

`cargo make local` or `cargo make local-quiet`

# .env

_google cloud sql proxy connections_
DATABASE_URL="postgres://username:pw@127.0.0.1:3306/jicloud"

_backend server<->server auth_
SHARED_SERVER_SECRET="[SECRET HERE]"

_name of file holding google cloud credentials_
GOOGLE_APPLICATION_CREDENTIALS="./secret-keys/ji-cloud-sandbox.json"

_JWT secret_
JWT_SECRET="[SECRET HERE]"

# CI/CD Setup

### SECRETS
* SLACK_BOT_TOKEN (the one that begins "xoxb-")
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY - *base64 encoded* json key for service account
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY_SANDBOX - same but for dev deployment


# Backend Architecture

## Code 

* Rust
* HTTP Framework: Warp
* DB Framework: Diesel (w/ migrations)

## Providers

* Google Cloud
  * SQL (PostgreSQL)
  * Storage
  * Cloud Run (and/or Functions, AppEngine, GCE, as needed)

* Firebase
  * Auth
  * Firestore

## CI/CD

* Github Actions
* Notifications to Slack