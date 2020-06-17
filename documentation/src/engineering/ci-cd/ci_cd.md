### Google Cloud 

1. Create a service account for ci/cd (e.g. `github-actions`)
2. Via the `IAM->Service Accounts` page, allow this service to operate as the service account for each cloud run instance
3. In cloud storage, give this service account cloud storage admin to buckets that are deployed via ci/cd:
   * frontend (release and sandbox)
   * storybook (sandbox)
   * docs (sandbox)

### Github Secrets

* SLACK_BOT_TOKEN (the one that begins "xoxb-")
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY - json key for service account
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY_SANDBOX - same but for dev deployment
* FIREBASE_TOKEN (run firebase login:ci)

The GOOGLE_CLOUD keys must be base64 encoded. Literally, take the json string and run it through a bas64 encoder.