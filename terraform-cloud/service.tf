resource "google_cloud_run_service" "api" {
  name     = "ji-cloud-api-staging"
  location = "europe-west1"

  template {
    spec {
      containers {
        image = "gcr.io/ji-cloud-developer-staging/ji-cloud-api-staging:latest"
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}

resource "google_cloud_run_service" "pages" {
  name     = "ji-cloud-pages-staging"
  location = "europe-west1"

  template {
    spec {
      containers {
        image = "gcr.io/ji-cloud-developer-staging/ji-cloud-pages-staging:latest"
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}

