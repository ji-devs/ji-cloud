import { argsToAttrs } from "@utils/attributes";
import "@elements/core/inputs/composed/inc-dec";

export default {
    title: "Core / Inputs / Composed"
}

interface Args {
    value: number,
    min: number,
    max: number,
}

const DEFAULT_ARGS: Args = {
    value: 3,
    min: -2,
    max: 4,
}

export const IncDec = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <input-inc-dec ${argsToAttrs(props)}></input-inc-dec>
    `;
}

IncDec.args = DEFAULT_ARGS;
