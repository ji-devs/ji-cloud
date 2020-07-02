# Secret Keys

`./_secret-keys` is a .gitignored folder containing the following, for local development only:

* `gcp-dev-release.json`: google cloud credentials for release project
* `gcp-dev-sandbox.json`: google cloud credentials for sandbox project

Generaly speaking, these should match the values in [.env](../dot-env/dot_env.md) and, if applicable, other [runtime settings](../runtime-settings/runtime_settings.md)

These secret keys should not be confused with the Google Secrets Manager keys, which are for production use (both release and sandbox)
