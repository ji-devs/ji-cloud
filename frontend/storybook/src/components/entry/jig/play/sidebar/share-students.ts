import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/sidebar/share-students";

export default {
    title: "Entry / Jig / Play / Sidebar"
}

interface Args {
    url: string,
    code: string,
}

const DEFAULT_ARGS:Args = {
    url: "ji.zone/play/3692",
    code: "3692",
}


export const ShareStudents = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-play-sidebar-share-students ${argsToAttrs(props)}>
            <button-text slot="back">< Back</button-text>
            <button-empty slot="close">&times;</button-empty>
            <button-text slot="copy-url">Copy URL</button-text>
            <button-text slot="copy-code">Copy Code</button-text>
        </jig-play-sidebar-share-students>
    `;
}
ShareStudents.args = DEFAULT_ARGS;
