# To create a new app

1. Add the directory to frontend/apps/entry/{NAME}
2. Add the entry to the Cargo.toml workspace
3. Add tasks in Workspace.toml (dev, sandbox, release, dev-nomedia)
4. Add Github actions for CI
5. Add routes that it will handle
6. Whitelist it in frontend/build-utils/src/dev-files.js

Scratch/local-only projects only need a subset of that:

1. Add the directory to frontend/apps/entry/{NAME}
2. Add the entry to the Cargo.toml workspace
3. Add task in Workspace.toml (just one for local dev)
4. Whitelist it in frontend/build-utils/src/dev-files.js