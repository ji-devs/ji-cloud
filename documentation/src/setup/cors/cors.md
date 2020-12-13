# CORS

For backend servers, it's configured as part of actix. 

For static media (including wasm), there is a script in `frontend/build-utils`. All that's needed is to `npm run` each of `cors:frontend`, `cors:media`, and `cors:uploads`

To configure the origins, see the respective `*-cors.json` file in the `build-utils` folder.

Since it runs `gsutil`, you may need to be careful to run it in a compatible shell (like cmd, not powershell, in windows)
