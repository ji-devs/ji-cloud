export const FRONTEND_SERVER_RELEASE = "https://frontend.jicloud.org";
export const FRONTEND_SERVER_SANDBOX = "https://frontend.sandbox.jicloud.org";

export const BUCKET_FRONTEND_RELEASE = "ji-cloud-frontend-origin-eu-001";
export const BUCKET_FRONTEND_SANDBOX = "ji-cloud-sandbox-frontend-origin-eu-001";
export const BUCKET_UPLOADS_RELEASE = "ji-cloud-uploads-origin-eu-001";
export const BUCKET_UPLOADS_SANDBOX = "ji-cloud-sandbox-uploads-origin-eu-001";
export const BUCKET_MEDIA = "ji-cloud-media-origin-eu-001";

const getMediaUrl = isDev => 
	isDev
        ? 'http://localhost:4102'
        : "https://media.jicloud.org";


export const getMediaUrl_UI = isDev =>
	`${getMediaUrl(isDev)}/ui`;

