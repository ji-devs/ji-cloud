# Google Cloud Setup

# Secrets

The runtime secrets are accessed via Gooogle Secret Manager. They are:

  * DB_PASS: the database password
  * INTER_SERVER: a random string to authenticate inter-server communication
  * JWT_SECRET: a random string used to sign JWT tokens
  * SANITY_TEST: not really necessary, just to make it easier to test that things are working correctly

### Backend - Cloud Run

1. Create a service account with a new name
2. Create an initial cloud run service and assign its service account to this new one
3. If Cloud SQL access is needed, assign it
4. In IAM - give the service account all the permissions it needs:
    * Cloud SQL Client
    * Compute Admin
    * Service Account Token Creator
    * Cloud Run Admin
    * Secret Manager Secret Accessor
5. If the Cloud Run instance needs to create files for a storage bucket - assign it as an admin for that bucket (via the bucket page)
6. Assign the custom domain

This is all the same regardless of language

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