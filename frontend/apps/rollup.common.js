require("dotenv").config({ path: __dirname + "/./../../.env" });

export function getEnv() {
    return {
        LOCAL_API_URL: process.env.LOCAL_API_URL,
        LOCAL_UPLOADS_URL: process.env.LOCAL_UPLOADS_URL,
        LOCAL_MEDIA_URL: process.env.LOCAL_MEDIA_URL,
        LOCAL_LEGACY_URL: process.env.LOCAL_LEGACY_URL,
        LOCAL_PAGES_URL: process.env.LOCAL_PAGES_URL,
        LOCAL_PAGES_URL_IFRAME: process.env.LOCAL_PAGES_URL_IFRAME,
        LOCAL_FRONTEND_URL: process.env.LOCAL_FRONTEND_URL,
        LOCAL_API_AUTH_OVERRIDE: process.env.LOCAL_API_AUTH_OVERRIDE,
    };
}
