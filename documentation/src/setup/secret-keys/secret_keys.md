# Secret Keys

`./_secret-keys` is a .gitignored folder containing the following, for local development only:

* `gcp-dev-release.json`: google cloud credentials for release project
* `gcp-dev-sandbox.json`: google cloud credentials for sandbox project

These should match the values in [config/.env](../config/config.md)

These secret keys should not be confused with the Google Secrets Manager keys, which are for production use (both release and sandbox)
