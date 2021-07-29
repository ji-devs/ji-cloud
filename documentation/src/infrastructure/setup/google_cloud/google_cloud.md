# Google Cloud Setup

# Secrets

The runtime secrets are accessed via Gooogle Secret Manager. 
Some of these are also in local .env

  * DB_PASS: the database password
  * INTER_SERVER: a random string to authenticate inter-server communication
  * JWT_SECRET: a random string used to sign JWT tokens
  * SANITY_TEST: not really necessary, just to make it easier to test that things are working correctly
  * ALGOLIA_KEY: algolia key
  * ALGOLIA_PROJECT_ID: algolia project id
  * GOOGLE_S3_ACCESS_KEY: the access key for accessing cloud storage like s3
  * GOOGLE_S3_ACCESS_SECRET: the access secret for accessing cloud storage like s3
  * SENTRY_DSN_API: Sentry API 
  * SENTRY_DSN_PAGES: Sentry Pages

### Backend - Cloud Run

1. Create a service account with a new name and:
  * the following roles (if not done here, add the account to IAM and do later)
    * Cloud SQL Client (optional)
    * Compute Admin
    * Service Account Token Creator
    * Cloud Run Admin
    * Secret Manager Secret Accessor
    * EventArc Admin
  * the CI service account as a user access (if not here, can be given `Service Account User` in permissions later)
2. Create an initial cloud run service and assign its service account to this new one 
    * If not part of the initial flow, edit and deploy a new revision and change in `Security` tab 
    * at this point the deploy will fail - CI will fix it later
3. If Cloud SQL access is needed, assign it
4. If the Cloud Run instance needs to create files for a storage bucket - assign it as an admin for that bucket (via the bucket page)
5. Assign the custom domain if it's exposed to the outside world


### Backend - Cloud Functions 

Used to purge FastlyCDN when certain buckets change

The domain names need to be set in index.js

### Backend - Compute Engine (for media sync)

Similar to Cloud Run, but the only access the service account needs besides compute engine is to the target media bucket

### Database

Remember to enable Cloud SQL Admin API


### Regions

Try to keep things in the same region, for example europe-west1

### Buckets

Every bucket has:
  * `allUsers` w/ Object Viewer permissions
  * Fastly CDN proxy
  * The service account assigned as Storage Admin if it needs write access
  * If not using a CDN, set the default index.html and 404.html (setting it anyway, via gsutil if not using an explicit domain as bucket name, doesn't hurt)

See [CI/CD](../ci-cd/ci_cd.md) for more detail