### Google Cloud 

1. Create a service account for ci/cd (e.g. `github-actions`)
2. Via the `IAM->Service Accounts` page, allow this service to operate as the service account for each cloud run instance
3. In cloud storage, give this service account cloud storage admin to buckets that are deployed via ci/cd:
   * frontend (release and sandbox)
   * storybook (sandbox)
   * docs (sandbox)
   * artifacts.project.appspot.com (release and sandbox, created by google - needed for cloud run deployments)
4. Also give the github actions service Cloud Run Admin and Cloud Functions Admin permissions 

Generally speaking, the very first deployment (see below) on a brand new project should be done manually via a local account first, before using ci/cd going forward.

In particular, the first cloud function deployment requires hitting "yes" on "Allow unauthenticated invocations"

### Github Secrets

* SLACK_BOT_TOKEN (the one that begins "xoxb-")
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY - json key for service account
* GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY_SANDBOX - same but for dev deployment
* FIREBASE_TOKEN (run firebase login:ci)

The GOOGLE_CLOUD keys must be base64 encoded. Literally, take the json string and run it through a bas64 encoder.

### Makefiles and Dockerfiles

Deployment is done via the top-level Makefile.toml as well as Dockerfiles as needed.

The PROJECT_ID and other variables are hardcoded directly in these files as needed (even if that's the process of setting as an env var)

If adjusting, remember to change sandbox vs. release :)
