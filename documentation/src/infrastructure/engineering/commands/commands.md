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

There are a few apps setup for scratch and showcase that are good for like a whiteboard on the dev side

# Storybook
Frontend wasm and storybook:
```
npm start
```

or

`npm run start:nomedia` (will not start the local media server)