import {argsToAttrs} from "@utils/attributes";
import "@elements/core/pills/pill-close";

export default {
    title: "Core / Pills"
}

interface Args {
    contents: string,
    negative: boolean,
}

const DEFAULT_ARGS:Args = {
    contents: "hello",
    negative: false
}

export const PillClose = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {contents, ...pillProps} = props

    return `<pill-close ${argsToAttrs(pillProps)}>${contents}</pill-close>`;
}

PillClose.args = DEFAULT_ARGS;