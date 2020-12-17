import {getMediaUrl_UI} from "@project-config";

const isDev = process.env["NODE_ENV"] === "development";

export const MEDIA_UI = getMediaUrl_UI(isDev);

export const mediaUi = (path:string):string => `${MEDIA_UI}/${path}`;