##
## Configures specified provider
## Doc: https://registry.terraform.io/providers/hashicorp/google/latest/docs/guides/provider_reference
##
provider "google" {
  project = "ji-cloud-developer-staging"
  region  = "europe-west1"
  zone    = "europe-west1-d"
}

resource "random_id" "db_name_suffix" {
  byte_length = 4
}

##
## Sql database instance
##
## Doc: https://registry.terraform.io/providers/hashicorp/google/latest/docs/resources/sql_database_instance
## Optional: Private, can specify VPC network
##
##
resource "google_sql_database_instance" "postgres" {
  provider         = google
  name             = "postgres-instance-${random_id.db_name_suffix.hex}"
  database_version = "POSTGRES_12" # Should keep consistent PostgreSQL version with Sandbox? 

  settings {
    availability_type = "REGIONAL"
    tier              = "db-f1-micro"
    backup_configuration {
      enabled    = true
      start_time = "00:00"
      backup_retention_settings {
        retained_backups = 7
      }
    }

    # Which networks we want to use 
    # ipv4_enabled or private
    ip_configuration {
      ipv4_enabled = true
    }
  }
}

##
## Sql database 
##
## Doc: https://registry.terraform.io/providers/hashicorp/google/latest/docs/resources/sql_database
##
##
resource "google_sql_database" "database" {
  name     = "ji-cloud"
  instance = google_sql_database_instance.postgres.name
}

##
## Creates new SQL user on user instance
##
## Doc: https://registry.terraform.io/providers/hashicorp/google/latest/docs/resources/sql_user
##
resource "google_sql_user" "database" {
  name     = "postgres"                                 # name of user
  instance = google_sql_database_instance.postgres.name # name of instance
  password = data.google_secret_manager_secret_version.basic.secret_data
}

##
## Fetches latest version of secret
##
data "google_secret_manager_secret_version" "basic" { 
  secret = "DB_PASSWORD"
}


