# Build Status

* ![Storybook](https://github.com/jewish-interactive/ji-cloud/workflows/Storybook/badge.svg)
* ![Api - Backend - Release](https://github.com/jewish-interactive/ji-cloud/workflows/Api%20-%20Backend%20-%20Release/badge.svg) / ![Api - Backend - Sandbox](https://github.com/jewish-interactive/ji-cloud/workflows/Api%20-%20Backend%20-%20Sandbox/badge.svg)
* ![ApiJs - Backend - Release](https://github.com/jewish-interactive/ji-cloud/workflows/ApiJs%20-%20Backend%20-%20Release/badge.svg) / ![ApiJs - Backend - Sandbox](https://github.com/jewish-interactive/ji-cloud/workflows/ApiJs%20-%20Backend%20-%20Sandbox/badge.svg)
* ![Pages - Backend - Release](https://github.com/jewish-interactive/ji-cloud/workflows/Pages%20-%20Backend%20-%20Release/badge.svg) / ![Pages - Backend - Sandbox](https://github.com/jewish-interactive/ji-cloud/workflows/Pages%20-%20Backend%20-%20Sandbox/badge.svg)
* ![User - Frontend - Release](https://github.com/jewish-interactive/ji-cloud/workflows/User%20-%20Frontend%20-%20Release/badge.svg) / ![User - Frontend - Sandbox](https://github.com/jewish-interactive/ji-cloud/workflows/User%20-%20Frontend%20-%20Sandbox/badge.svg)

# Secrets

* SLACK_BOT_TOKEN (the one that begins "xoxb-")
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY - json key for service account
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY_SANDBOX - same but for dev deployment
* FIREBASE_TOKEN (run firebase login:ci)

The GOOGLE_CLOUD keys must be base64 encoded. Literally, take the json string and run it through a bas64 encoder.

Also the service accounts must have the required permissions for Google Cloud Run, Google Cloud Storage, etc.

