# Google Cloud Setup

### Buckets

Every bucket has:
  * `allUsers` w/ Object Viewer permissions
  * Fastly CDN proxy
  * The service account assigned as Storage Admin if it needs write access
    * github deployment 
      * frontend (release and sandbox)
      * storybook
      * docs
    * api
      * uploads (release and sandbox)


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

### Ci/Cd and Github Integration

1. Create a service account for github actions
2. For backend: Assign this service account to the cloud run instance created above
3. For frontend: Assign this service account as an admin of the bucket it deploys to