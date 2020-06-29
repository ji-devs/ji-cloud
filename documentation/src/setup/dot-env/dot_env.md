# Dot env files

## Backend and db_migrations

Each backend project folder, as well as `build-utils/db_migrations` needs a `.env` file with:

```
GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX="[PATH-TO-SANDBOX-SERVICE-ACCOUNT-JSON]"
GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE="[PATH-TO-RELEASE-SERVICE-ACCOUNT-JSON]"
LOCAL_DB_USER="[LOCAL DATABASE USERNAME]"
LOCAL_DB_PASS="[LOCAL DATABASE PASS]"
LOCAL_DB_PORT="[LOCAL DATABASE PORT]"
LOCAL_DB_NAME="[LOCAL DATABASE NAME]"
```

The path to the JSON must be escaped for windows, e.g.:

```
GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX="E:\\ji\\ji-cloud\\_secret-keys\\gcp-developer-sandbox.json"
```

## Build Utils

The `build-utils` folder needs a `.env` file with:

```

LOCAL_CDN_MEDIA_DIR="[PATH-TO-MEDIA-CDN-FOLDER]"
LOCAL_CDN_PORT=4102

REMOTE_CDN_MEDIA_BUCKET="[MEDIA_BUCKET]"
```

The `REMOTE_CDN_MEDIA_BUCKET` isn't actually used right now due to the [media sync strategy](../media/media.md)

