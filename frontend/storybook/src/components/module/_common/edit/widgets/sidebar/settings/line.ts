import {argsToAttrs} from "@utils/attributes";
import {Kind} from "@elements/module/_common/edit/widgets/settings/line";
import "@elements/module/_common/edit/widgets/settings/line";
import {mapToString, arrayCount} from "@utils/array";
import {Button} from "./button";

const kinds:Array<Kind> = ["card-view"]

export default {
    title: "Module / _COMMON /  edit / Widgets / Sidebar / Settings "
}

interface Args {
    kind: Kind,
}

const DEFAULT_ARGS:Args = {
    kind: "card-view",
}

export const Line = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
            <module-settings-line ${argsToAttrs(props)}>
	    	    ${Button({offsetContainer: false})} ${Button({offsetContainer: false})}
            </module-settings-line>
    `;
}

Line.args = DEFAULT_ARGS;

Line.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: kinds
        }
    },
}