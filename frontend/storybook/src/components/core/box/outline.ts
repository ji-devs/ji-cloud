import { argsToAttrs } from "@utils/attributes";
import "@elements/core/box/outline-container";
import { Color } from "@elements/core/box/outline-container";

export default {
    title: "Core / Box ",
};
interface Args {
    color: Color;
    borderInside: boolean;
    uncloseable: boolean;
}

const DEFAULT_ARGS: Args = {
    color: "blue",
    borderInside: false,
    uncloseable: false,
};

export const OutlineContainer = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
	<div style="display: flex; background-color: #ffe1a7; flex-direction: column; margin: 50px; gap: 30px;">
    		<box-outline-container ${argsToAttrs(
                props
            )}><div>Hello World</div></box-outline-container>
    		<box-outline-container ${argsToAttrs(
                props
            )} thick><div>Hello World</div></box-outline-container>
    		<box-outline-container ${argsToAttrs(
                props
            )}><div>Hello World</div></box-outline-container>
	</div>
	`;
};

OutlineContainer.args = DEFAULT_ARGS;

OutlineContainer.argTypes = {
    color: {
        control: {
            type: "inline-radio",
            options: ["blue"],
        },
    },
};
