
export const STAGE_EDIT = {
	width: 1920,
	height: 1080,
	paddingX: 0,
	paddingY: 0,
        marginX: 40, //Always has this margin
	marginY: 0
};

export const STAGE_PLAYER = {
	width: 1920,
	height: 1080,
	paddingX: 0,
	paddingY: 0,
	marginX: 0,
	marginY: 0
};
export const STAGE_LEGACY = {
	width: 1024,
	height: 768,
	paddingX: 0,
	paddingY: 0,
	marginX: 0,
	marginY: 0
};


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


export const getMediaUrl = (isDev:boolean):string => {
        return isDev
                ? `http://localhost:4102`
                : URL_MEDIA;
}

export const getMediaUrl_UI = (isDev:boolean):string => {
        return `${getMediaUrl(isDev)}/ui`;
}


export const getMediaUrl_UPLOADS = (deployTarget: string | undefined):string => {
        switch(deployTarget) {
                case "local": return "http://localhost:9000/test-bucket";
                case "sandbox": return URL_UPLOADS_SANDBOX;
                case "release": return URL_UPLOADS_RELEASE;
                default: return "";
        }
}

