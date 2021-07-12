import {argsToAttrs} from "@utils/attributes";
import "@elements/core/box/outline";
import {Color} from "@elements/core/box/outline";

export default {
  title: 'Core / Box ',
}
interface Args {
    color: Color,
    borderInside: boolean,
    uncloseable: boolean,
}

const DEFAULT_ARGS:Args = {
    color: "blue",
    borderInside: false, 
    uncloseable: false, 
}

export const Outline = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
	<div style="display: flex; background-color: #ffe1a7; flex-direction: column; margin: 50px; gap: 30px;">
    		<box-outline ${argsToAttrs(props)}><div>Hello World</div></box-outline>
    		<box-outline ${argsToAttrs(props)} thick><div>Hello World</div></box-outline>
    		<box-outline ${argsToAttrs(props)}><div>Hello World</div></box-outline>
	</div>
	`;
}

Outline.args = DEFAULT_ARGS;

Outline.argTypes = {
	color: {
		control: {
			type: 'inline-radio',
			options: ["blue"]
		}
	}
}