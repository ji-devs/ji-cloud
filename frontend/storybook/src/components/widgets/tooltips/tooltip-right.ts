import {argsToAttrs} from "@utils/attributes";
import "@elements/widgets/tooltips/right";
import {Kind as TooltipKind} from "@elements/widgets/tooltips/right";
import {Padding} from "@elements/widgets/tooltips/right";
export default {
    title: "Widgets / Tooltips"
}

interface Args {
    kind: TooltipKind,
    contents: string,
    padding:Padding,
}

const DEFAULT_ARGS:Args = {
    kind: "success",
    contents: "hello",
    padding:"small",
}

export const Right = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {contents, ...tooltipProps} = props

    return `<tooltip-right ${argsToAttrs(tooltipProps)}>${contents}</tooltip-right>`;
}

Right.args = DEFAULT_ARGS;

Right.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["error", "success", "plain"]
        }
    }
}