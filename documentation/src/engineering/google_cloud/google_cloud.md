# Google Cloud Setup

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

### Backend - Cloud Functions

TODO

### Regions

Try to keep things in the same region, for example europe-west1

### Buckets

Every bucket has:
  * `allUsers` w/ Object Viewer permissions
  * Fastly CDN proxy
  * The service account assigned as Storage Admin if it needs write access

See [CI/CD](../ci-cd/ci_cd.md) for more detail