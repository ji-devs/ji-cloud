import {argsToAttrs} from "@utils/attributes";
import "@elements/core/tooltips/error";

export default {
    title: "Core / Tooltips"
}

interface Args {
    body: string
}

const DEFAULT_ARGS:Args = {
    body: "Body here"
}

export const Error = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
    <i>Note: the arrow positioning will only be driven at runtime</i>
    <br/>
    <tooltip-error ${argsToAttrs(props)}>
        Content Here
    </tooltip-confirm>
    `;
}

Error.args = DEFAULT_ARGS;
