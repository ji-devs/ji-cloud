
##
## Cloud Storage Buckets
##
## Doc: https://registry.terraform.io/providers/hashicorp/google/latest/docs/resources/storage_bucket
## Note: Once bucket is created, location cannot be changed
##
resource "google_storage_bucket" "frontend" {
  name          = "ji-cloud-staging-frontend-origin-eu-001"
  location      = "europe-west1"

  uniform_bucket_level_access = true

  website {
    main_page_suffix = "index.html"
    not_found_page   = "404.html"
  }
  cors {
    origin          = ["*"]
    method          = ["GET", "HEAD", "OPTIONS"]
    response_header = ["*"]
    max_age_seconds = 3600
  }
}

resource "google_storage_bucket" "processing" {
  name          = "ji-cloud-staging-processing-eu-001"
  location      = "europe-west1"

  uniform_bucket_level_access = true

}

resource "google_storage_bucket" "uploads" {
  name          = "ji-cloud-staging-uploads-origin-eu-001"
  location      = "europe-west1"

  uniform_bucket_level_access = true

  website {
    main_page_suffix = "index.html"
    not_found_page   = "404.html"
  }
  cors {
    origin          = ["*"]
    method          = ["GET", "HEAD", "PUT", "DELETE", "POST", "OPTIONS"]
    response_header = ["*"]
    max_age_seconds = 1
  }
}


