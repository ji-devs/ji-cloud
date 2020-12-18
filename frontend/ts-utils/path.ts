import {getMediaUrl_UI, getMediaUrl_UPLOADS} from "@project-config";

const isDev = process.env["NODE_ENV"] === "development";
const deployTarget = process.env["DEPLOY_TARGET"] || process.env["STORYBOOK_DEPLOY_TARGET"];

export const MEDIA_UI = getMediaUrl_UI(isDev);
export const MEDIA_UPLOADS = getMediaUrl_UPLOADS(deployTarget);

export const mediaUi = (path:string):string => `${MEDIA_UI}/${path}`;
const mediaUploads = (path:string):string => `${MEDIA_UPLOADS}/${path}`;

export type MediaLibOptions = "global" | "user" | "web";
export type MediaSizeOptions = "original" | "full" | "thumb";

const imagePrefix = (lib:MediaLibOptions):string => {
    switch(lib) {
        case "global": return "image";
        case "user": return "image-user";
        case "web": return "image-web";
    }
}

const audioPrefix = (lib:MediaLibOptions):string => {
    switch(lib) {
        case "global": return "audio/global";
        case "user": return "audio/user";
        case "web": return "audio/web";
    }
}

const sizeVariant = (size:MediaSizeOptions):string => {
    switch(size) {
        case "original": return "original";
        case "full": return "resized";
        case "thumb": return "thumbnail";
    }
}

export const imageLib = ({lib, size, id}:{lib: MediaLibOptions, size: MediaSizeOptions, id: string}) => {
    const prefix = imagePrefix(lib);
    const variant = sizeVariant(size);

    return mediaUploads(`${prefix}/${variant}/${id}`);
}