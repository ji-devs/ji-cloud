import {argsToAttrs} from "@utils/attributes";
import "@elements/core/pills/pill-close";

export default {
    title: "Core / Pills"
}

interface Args {
    label: string,
    negative: boolean,
}

const DEFAULT_ARGS:Args = {
    label: "hello",
    negative: false
}

export const PillClose = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<pill-close ${argsToAttrs(props)}></pill-close>`;
}

PillClose.args = DEFAULT_ARGS;
