Each project may have its own custom settings, like booleans for debug options, but there are global config options which apply everywhere. These reside in the top-level `config/` directory.

# config/js 

In order to make these config settings available, it needs to be rebuilt with `npm run build`

# config/rust

Nothing to do (other than edit the values).

# .env 

There exists a top-level `config/.env`. Since this isn't checked into the repo, it must be filled in manually after cloning.

Note that windows paths should be escaped, e.g. `C:\\ji\\ji-cloud\\_secret-keys\\gcp-dev-sandbox.json`

```
GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX="[PATH_TO_SANDBOX_JSON]"
GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE="[PATH_TO_RELEASE_JSON]"
```

_Make sure to match with the command to systemfd in backend/api/main/Makefile.toml_

```
LOCAL_API_PORT=8081
```

_Make sure to match with the command to systemfd in backend/pages/Makefile.toml_

```
LOCAL_PAGES_PORT=8080
```

```
LOCAL_API_JS_PORT=8082

LOCAL_DB_USER="[LOCAL_DB_USER]"
LOCAL_DB_PASS="[LOCAL_DB_PASS]"
LOCAL_DB_PORT=[LOCAL_DB_PORT]
LOCAL_DB_NAME="[LOCAL_DB_NAME]"

DATABASE_URL="postgres://[LOCAL_DB_USER]:[LOCAL_DB_PASS]@localhost:[LOCAL_DB_PORT]/[LOCAL_DB_NAME]"

POSTGRES_PASSWORD=[LOCAL_DB_PASS]
POSTGRES_USER=[LOCAL_DB_USER]
POSTGRES_DB=[LOCAL_DB_NAME]

LOCAL_CDN_MEDIA_DIR="[PATH_TO_MEDIA_DIR]"
LOCAL_CDN_MEDIA_PORT=4102

LOCAL_CDN_FRONTEND_DIR="[PATH_TO_FRONTEND_DIR]"
LOCAL_CDN_FRONTEND_PORT=4103
```