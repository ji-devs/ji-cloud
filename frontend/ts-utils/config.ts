const constants = require("../config/constants");

const {
        URL_MEDIA, 
        URL_UPLOADS_RELEASE, 
        URL_UPLOADS_SANDBOX,
        STAGE_PLAYER, 
        STAGE_EDIT, 
        STAGE_LEGACY,
} = constants;

//Re-exports
export {STAGE_PLAYER, STAGE_EDIT, STAGE_LEGACY};

const getUrl = (envKey:string, fallback: string):string => {
	const value = (process as any).env[envKey];
	return value == undefined || value == "" ? fallback : value;
}

export const getMediaUrl = (isDev:boolean):string => {
        return isDev
                ? getUrl("LOCAL_MEDIA_URL", "http://localhost:4102")
                : URL_MEDIA;
}

export const getMediaUrl_UI = (isDev:boolean):string => {
        return `${getMediaUrl(isDev)}/ui`;
}


export const getMediaUrl_UPLOADS = (deployTarget: string | undefined):string => {
        switch(deployTarget) {
                case "local": return getUrl("LOCAL_UPLOADS_URL", "http://localhost:9000/test-bucket");
                case "sandbox": return URL_UPLOADS_SANDBOX;
                case "release": return URL_UPLOADS_RELEASE;
                default: return "";
        }
}

