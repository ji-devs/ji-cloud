'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

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

exports.BUCKET_FRONTEND_RELEASE = BUCKET_FRONTEND_RELEASE;
exports.BUCKET_FRONTEND_SANDBOX = BUCKET_FRONTEND_SANDBOX;
exports.BUCKET_MEDIA = BUCKET_MEDIA;
exports.BUCKET_UPLOADS_RELEASE = BUCKET_UPLOADS_RELEASE;
exports.BUCKET_UPLOADS_SANDBOX = BUCKET_UPLOADS_SANDBOX;
exports.URL_DOCS = URL_DOCS;
exports.URL_FRONTEND_RELEASE = URL_FRONTEND_RELEASE;
exports.URL_FRONTEND_SANDBOX = URL_FRONTEND_SANDBOX;
exports.URL_MEDIA = URL_MEDIA;
exports.URL_STORYBOOK_RELEASE = URL_STORYBOOK_RELEASE;
exports.URL_STORYBOOK_SANDBOX = URL_STORYBOOK_SANDBOX;
exports.getMediaUrl_UI = getMediaUrl_UI;
