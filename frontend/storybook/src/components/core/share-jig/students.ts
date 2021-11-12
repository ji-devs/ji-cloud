import { argsToAttrs } from "@utils/attributes";
import "@elements/core/share-jig/students";

export default {
    title: "Core / Share jig",
};

interface Args {
    url: string;
    code: string;
}

const DEFAULT_ARGS: Args = {
    url: "ji.zone/play/3692",
    code: "3692",
};

export const Students = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <share-jig-students ${argsToAttrs(props)}>
            <button-rect kind="text" slot="back">< Back</button-rect>
            <button-empty slot="close">&times;</button-empty>
            <button-rect kind="text" slot="copy-url">Copy URL</button-rect>
            <button-rect kind="text" slot="copy-code">Copy Code</button-rect>
        </share-jig-students>
    `;
};
Students.args = DEFAULT_ARGS;
