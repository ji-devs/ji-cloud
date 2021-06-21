import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/sidebar/share-main";

export default {
    title: "Entry / Jig / Play / Sidebar"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}


export const ShareMain = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-play-sidebar-share-main ${argsToAttrs(props)}>
            <button-empty slot="close">&times;</button-empty>
            <jig-play-sidebar-share-option kind="students"></jig-play-sidebar-share-option>
            <jig-play-sidebar-share-option kind="embed"></jig-play-sidebar-share-option>
            <jig-play-sidebar-share-option kind="copy"></jig-play-sidebar-share-option>
        </jig-play-sidebar-share-main>
    `;
}
ShareMain.args = DEFAULT_ARGS;
