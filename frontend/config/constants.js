module.exports = {
	STAGE_EDIT: {
		width: 1920,
		height: 1080,
		paddingX: 0,
		paddingY: 0,
		marginX: 40, //Always has this margin
		marginY: 0
	},

	STAGE_PLAYER: {
		width: 1920,
		height: 1080,
		paddingX: 0,
		paddingY: 0,
		marginX: 0,
		marginY: 0
	},

	STAGE_LEGACY: {
		width: 1024,
		height: 768,
		paddingX: 0,
		paddingY: 0,
		marginX: 0,
		marginY: 0
	},


	BUCKET_FRONTEND_RELEASE: "ji-cloud-frontend-origin-eu-001",
	BUCKET_FRONTEND_SANDBOX: "ji-cloud-sandbox-frontend-origin-eu-001",
	BUCKET_UPLOADS_RELEASE: "ji-cloud-uploads-origin-eu-001",
	BUCKET_UPLOADS_SANDBOX: "ji-cloud-sandbox-uploads-origin-eu-001",
	BUCKET_MEDIA: "ji-cloud-media-origin-eu-001",
	BUCKET_LEGACY: "ji-cloud-legacy-eu-001",

//is actually on sandbox (since it's for devs)
	URL_DOCS: "https://docs.jicloud.org",
//is actually on release (since it's for public)
	URL_MEDIA: "https://media.jicloud.org",
	URL_LEGACY: "https://legacy.jicloud.org",
	URL_FRONTEND_RELEASE: "https://frontend.jicloud.org",
	URL_FRONTEND_SANDBOX: "https://frontend.sandbox.jicloud.org",
	URL_STORYBOOK_RELEASE: "https://storybook.jicloud.org",
	URL_STORYBOOK_SANDBOX: "https://storybook.sandbox.jicloud.org",
	URL_UPLOADS_RELEASE: "https://uploads.jicloud.org",
	URL_UPLOADS_SANDBOX: "https://uploads.sandbox.jicloud.org"
};