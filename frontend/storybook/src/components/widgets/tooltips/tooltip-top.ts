import {argsToAttrs} from "@utils/attributes";
import "@elements/widgets/tooltips/top";
import {Kind as TooltipKind} from "@elements/widgets/tooltips/top";

export default {
    title: "Widgets / Tooltips"
}

interface Args {
    kind: TooltipKind,
    contents: string,
}

const DEFAULT_ARGS:Args = {
    kind: "error",
    contents: "hello"
}

export const Top = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {contents, ...tooltipProps} = props

    return `<tooltip-top ${argsToAttrs(tooltipProps)}>${contents}</tooltip-tip>`;
}

Top.args = DEFAULT_ARGS;

Top.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["error"]
        }
    }
}