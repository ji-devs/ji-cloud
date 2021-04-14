import {argsToAttrs} from "@utils/attributes";
import "@elements/core/tooltips/info";

export default {
    title: "Core / Tooltips"
}

interface Args {
    title: string,
    body: string
}

const DEFAULT_ARGS:Args = {
    title: "Title here",
    body: "Body here"
}

export const Info = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
    <i>Note: the arrow positioning will only be driven at runtime</i>
    <br/>
    <tooltip-info ${argsToAttrs(props)} showId="debug" closeable>
        Content Here
    </tooltip-info>
    `;
}

Info.args = DEFAULT_ARGS;
