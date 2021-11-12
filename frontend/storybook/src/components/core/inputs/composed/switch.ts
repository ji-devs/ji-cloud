import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/composed/switch";

export default {
    title: "Core / Inputs / Composed",
};

interface Args {
    label: string;
    enabled: boolean;
}

const DEFAULT_ARGS: Args = {
    label: "hello",
    enabled: false,
};

export const Switch = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    const {} = props;

    return `<input-switch ${argsToAttrs(props)}></input-switch>`;
};

Switch.args = DEFAULT_ARGS;
