
export const STAGE_WIDTH = 1920;
export const STAGE_HEIGHT = 1080;
export const STAGE_PADDING_Y_PERC = 0.0; // in percentage, to offset the stage area a bit
export const STAGE_PADDING_X_PERC = 0.0;

export const BUCKET_FRONTEND_RELEASE = "ji-cloud-frontend-origin-eu-001";
export const BUCKET_FRONTEND_SANDBOX = "ji-cloud-sandbox-frontend-origin-eu-001";
export const BUCKET_UPLOADS_RELEASE = "ji-cloud-uploads-origin-eu-001";
export const BUCKET_UPLOADS_SANDBOX = "ji-cloud-sandbox-uploads-origin-eu-001";
export const BUCKET_MEDIA = "ji-cloud-media-origin-eu-001";

//is actually on sandbox (since it's for devs)
export const URL_DOCS = "https://docs.jicloud.org";
//is actually on release (since it's for public)
export const URL_MEDIA = "https://media.jicloud.org";
export const URL_FRONTEND_RELEASE = "https://frontend.jicloud.org";
export const URL_FRONTEND_SANDBOX = "https://frontend.sandbox.jicloud.org";
export const URL_STORYBOOK_RELEASE = "https://storybook.jicloud.org";
export const URL_STORYBOOK_SANDBOX = "https://storybook.sandbox.jicloud.org";
export const URL_UPLOADS_RELEASE = "https://uploads.jicloud.org";
export const URL_UPLOADS_SANDBOX = "https://uploads.sandbox.jicloud.org";

const getMediaUrl = (isDev:boolean):string => 
	isDev
        ? `http://localhost:4102`
        : URL_MEDIA;


export const getMediaUrl_UI = (isDev:boolean):string =>
	`${getMediaUrl(isDev)}/ui`;

