import {argsToAttrs} from "@utils/attributes";
import "@elements/core/popups/popup-body";

export default {
    title: "Core / Popup"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Confirm = (props?:Args) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <popup-body ${argsToAttrs(props)}>
            <button-text slot="back">Back to JIG settings</button-text>
            <button-icon icon="x" slot="close"></button-icon>
            <h2 slot="heading">Add Background Music</h2>
            <div slot="body">
                body
            </div>
        </popup-body">
    `;
}

Confirm.args = DEFAULT_ARGS;
