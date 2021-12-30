# Start projects in dev mode

Inside the `frontend/apps` folder:

```
cargo make [target] [app]
```

Where `target` is one of:
* `local-main`
* `local-iframe` (will use a different port)
* `local-main-nomedia` (will not start the local media server)
* `local-iframe-nomedia` (different port and will not start the local media server)

`app` is the name of the SPA (`user`, `jig/edit`, `module/memory/edit`, etc.)

## Available apps and modules

- [`frontend/apps/entry`](https://github.com/ji-devs/ji-cloud/tree/sandbox/frontend/apps/crates/entry)
- [`frontend/apps/entry/module`](https://github.com/ji-devs/ji-cloud/tree/sandbox/frontend/apps/crates/entry/module)
  - Format `module/{module}/{edit|play}`. Example `module/cover/edit`

There are a few apps setup for scratch and showcase that are good for like a whiteboard on the dev side

# Storybook
Frontend wasm and storybook:
```
npm start
```

or

`npm run start:nomedia` (will not start the local media server)
