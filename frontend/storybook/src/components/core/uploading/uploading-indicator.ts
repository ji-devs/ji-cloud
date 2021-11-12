import { argsToAttrs } from "@utils/attributes";
import "@elements/core/uploading/uploading-indicator";

export default {
    title: "Core / Uploading Indicator",
};

interface Args {
    progress: number;
}

const DEFAULT_ARGS: Args = {
    progress: 38,
};

export const UploadingIndicator = (props?: Args) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <uploading-indicator ${argsToAttrs(props)}>
            <button-icon slot="cancel" icon="x"></button-icon>
        </uploading-indicator>
    `;
};

UploadingIndicator.args = DEFAULT_ARGS;
