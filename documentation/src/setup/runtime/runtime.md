# Runtime settings

The runtime settings create dynamic strings used for media url, database connection string, etc.

Some of the values are pulled from outside env or secret vars, and others are hardcoded.

The hardcoded runtime values are all consolidated so that editing is made easy:

  * Backend: `shared/rust/src/backend/settings.rs`
  * Frontend: `shared/rust/src/frontend/settings.rs`

Each project may additionally have its own custom settings, like booleans for debug options, usually building on the above settings (or [env vars](../dot-env/dot_env.md) etc.). However, these aren't in the same category of configurable strings and directories.

Also, some sub-projects have their own proprietary settings (such as [storybook](../storybook/storybook.md))