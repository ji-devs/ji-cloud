const BUCKET_FRONTEND_RELEASE = "ji-cloud-frontend-origin-eu-001";
const BUCKET_FRONTEND_SANDBOX = "ji-cloud-sandbox-frontend-origin-eu-001";
const BUCKET_UPLOADS_RELEASE = "ji-cloud-uploads-origin-eu-001";
const BUCKET_UPLOADS_SANDBOX = "ji-cloud-sandbox-uploads-origin-eu-001";
const BUCKET_MEDIA = "ji-cloud-media-origin-eu-001";

//is actually on sandbox (since it's for devs)
const URL_DOCS = "https://docs.jicloud.org";
//is actually on release (since it's for public)
const URL_MEDIA = "https://media.jicloud.org";
const URL_FRONTEND_RELEASE = "https://frontend.jicloud.org";
const URL_FRONTEND_SANDBOX = "https://frontend.sandbox.jicloud.org";
const URL_STORYBOOK_RELEASE = "https://storybook.jicloud.org";
const URL_STORYBOOK_SANDBOX = "https://storybook.sandbox.jicloud.org";


const getMediaUrl = isDev => 
	isDev
        ? 'http://localhost:4102'
        : "https://media.jicloud.org";


const getMediaUrl_UI = isDev =>
	`${getMediaUrl(isDev)}/ui`;

export { BUCKET_FRONTEND_RELEASE, BUCKET_FRONTEND_SANDBOX, BUCKET_MEDIA, BUCKET_UPLOADS_RELEASE, BUCKET_UPLOADS_SANDBOX, URL_DOCS, URL_FRONTEND_RELEASE, URL_FRONTEND_SANDBOX, URL_MEDIA, URL_STORYBOOK_RELEASE, URL_STORYBOOK_SANDBOX, getMediaUrl_UI };
