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
            <button-rect kind="text" slot="back">< Back</button-rect>
            <button-empty slot="close">&times;</button-empty>
            <button-rect kind="text" slot="copy-url">Copy URL</button-rect>
            <button-rect kind="text" slot="copy-code">Copy Code</button-rect>
        </jig-play-sidebar-share-students>
    `;
}
ShareStudents.args = DEFAULT_ARGS;
