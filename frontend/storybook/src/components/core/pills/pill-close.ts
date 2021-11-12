import { argsToAttrs } from "@utils/attributes";
import "@elements/core/pills/pill-close";
import "@elements/core/pills/pill-close-delete";

export default {
    title: "Core / Pills",
};

interface Args {
    label: string;
}

const DEFAULT_ARGS: Args = {
    label: "hello",
};

export const PillClose = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <pill-close ${argsToAttrs(props)}>
            <pill-close-delete slot="delete"></pill-close-delete>
        </pill-close>
    `;
};

PillClose.args = DEFAULT_ARGS;
