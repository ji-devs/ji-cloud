##
## Configures specified provider
## Doc: https://registry.terraform.io/providers/hashicorp/google/latest/docs/guides/provider_reference
##
provider "google" {
  project = "ji-cloud-developer-staging"
  region  = "europe-west1"
  zone    = "europe-west1-d"
}