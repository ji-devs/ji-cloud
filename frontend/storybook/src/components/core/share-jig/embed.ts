import {argsToAttrs} from "@utils/attributes";
import "@elements/core/share-jig/embed";

export default {
    title: "Core / Share jig"
}

interface Args {
    value: string,
}

const DEFAULT_ARGS:Args = {
    value: "<iframe src='media/webplayer/webplayer.html?structureJson=https://jitap.net/store/api/album/14554/structure&v=3.4.5&emded=true' webkitallowfullscreen=‘’ mozallowfullscreen=‘’ allowfullscreen=‘’ style=‘border:0’></iframe>",
}


export const Embed = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <share-jig-embed ${argsToAttrs(props)}>
            <button-rect kind="text" slot="back">< Back</button-rect>
            <button-empty slot="close">&times;</button-empty>
            <button-rect kind="text" slot="copy">Copy code</button-rect>
        </share-jig-embed>
    `;
}
Embed.args = DEFAULT_ARGS;
