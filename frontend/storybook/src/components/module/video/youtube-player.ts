import {argsToAttrs} from "@utils/attributes";
import "@elements/module/video/youtube-player";


export default {
    title: "Module / Video / play" 
}

interface Args {
    videoId: string,
    autoplay: boolean,
    loop: boolean,
}

const DEFAULT_ARGS:Args = {
    videoId: "UQosz5VNsjY",
    autoplay: true,
    loop: true,
}

export const YoutubePlayer = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <video-youtube-player ${argsToAttrs(props)}></video-youtube-player>
    `;
}

YoutubePlayer.args = DEFAULT_ARGS;
