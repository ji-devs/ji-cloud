import {getMediaUrl_UI} from "../../../config/js"; 

const isDev = process.env["NODE_ENV"] === "development";

const MEDIA_UI = getMediaUrl_UI(isDev);

export function renderTemplate(template, data) {
	const base = template.replace(/%MEDIA_UI%/g, MEDIA_UI);
	
	if(data == null) {
		return base;
	} else {
		return interpolate(base, data);
	}
}

//See: https://stackoverflow.com/a/47358102
const regex = /\${[^{]+}/g;

export function interpolate(template, variables, fallback) {
    return template.replace(regex, (match) => {
        const path = match.slice(2, -1).trim();
        return getObjPath(path, variables, fallback);
    });
}

//get the specified property or nested property of an object
function getObjPath(path, obj, fallback = '') {
    return path.split('.').reduce((res, key) => res[key] || fallback, obj);
}
