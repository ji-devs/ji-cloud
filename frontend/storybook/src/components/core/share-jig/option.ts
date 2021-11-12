import { argsToAttrs } from "@utils/attributes";
import "@elements/core/share-jig/option";
import { Kind } from "@elements/core/share-jig/option";

export default {
    title: "Core / Share jig",
};

interface Args {
    kind: Kind;
}

const DEFAULT_ARGS: Args = {
    kind: "students",
};

export const Option = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <share-jig-option ${argsToAttrs(props)}></share-jig-option>
    `;
};
Option.args = DEFAULT_ARGS;
Option.argTypes = {
    kind: {
        control: {
            type: "inline-radio",
            options: ["students", "embed", "copy"],
        },
    },
};
