import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/sidebar/share-embed";

export default {
    title: "Entry / Jig / Play / Sidebar"
}

interface Args {
    value: string,
}

const DEFAULT_ARGS:Args = {
    value: "<iframe src='media/webplayer/webplayer.html?structureJson=https://jitap.net/store/api/album/14554/structure&v=3.4.5&emded=true' webkitallowfullscreen=‘’ mozallowfullscreen=‘’ allowfullscreen=‘’ style=‘border:0’></iframe>",
}


export const ShareEmbed = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-play-sidebar-share-embed ${argsToAttrs(props)}>
            <button-rect kind="text" slot="back">< Back</button-rect>
            <button-empty slot="close">&times;</button-empty>
            <button-rect kind="text" slot="copy">Copy code</button-rect>
        </jig-play-sidebar-share-embed>
    `;
}
ShareEmbed.args = DEFAULT_ARGS;
