[![Build Status](https://github.com/jewish-interactive/ji-cloud-api-js/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/jewish-interactive/ji-cloud-api-js/actions)

# About

This is the ji cloud api stuff that made sense to write in JS instead of Rust

# CI/CD Setup

### SECRETS
* SLACK_BOT_TOKEN (the one that begins "xoxb-")
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY - *base64 encoded* json key for service account
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY_DEV - same but for dev deployment