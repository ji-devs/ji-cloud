const FRONTEND_SERVER_RELEASE = "https://frontend.jicloud.org";
const FRONTEND_SERVER_SANDBOX = "https://frontend.sandbox.jicloud.org";

const BUCKET_FRONTEND_RELEASE = "ji-cloud-frontend-origin-eu-001";
const BUCKET_FRONTEND_SANDBOX = "ji-cloud-sandbox-frontend-origin-eu-001";
const BUCKET_UPLOADS_RELEASE = "ji-cloud-uploads-origin-eu-001";
const BUCKET_UPLOADS_SANDBOX = "ji-cloud-sandbox-uploads-origin-eu-001";
const BUCKET_MEDIA = "ji-cloud-media-origin-eu-001";

const getMediaUrl = isDev => 
	isDev
        ? 'http://localhost:4102'
        : "https://media.jicloud.org";


const getMediaUrl_UI = isDev =>
	`${getMediaUrl(isDev)}/ui`;

export { BUCKET_FRONTEND_RELEASE, BUCKET_FRONTEND_SANDBOX, BUCKET_MEDIA, BUCKET_UPLOADS_RELEASE, BUCKET_UPLOADS_SANDBOX, FRONTEND_SERVER_RELEASE, FRONTEND_SERVER_SANDBOX, getMediaUrl_UI };
