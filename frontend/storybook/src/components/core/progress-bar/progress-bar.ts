import "@elements/core/progress-bar/progress-bar";
import { ProgressColor } from "@elements/core/progress-bar/progress-bar";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Core / Progress Bar",
};

interface Args {
    progress: number | "infinite";
    color: ProgressColor;
}

const DEFAULT_ARGS: Args = {
    progress: 50,
    color: "blue",
};

export const ProgressBar = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <div style="height: 10px; margin: 25px; width: 350px;">
            <progress-bar ${argsToAttrs(props)}>
                <div slot="progress-label">Processing...</div>
            </progress-bar>
        </div>
    `;
};

ProgressBar.args = DEFAULT_ARGS;

ProgressBar.argTypes = {
    color: {
        control: {
            type: "inline-radio",
            options: ["blue", "green"],
        },
    },
};
