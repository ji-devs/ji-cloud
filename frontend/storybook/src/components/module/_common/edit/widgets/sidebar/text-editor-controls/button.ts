import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/edit/widgets/text-editor-controls/button";
import { Kind } from "@elements/module/_common/edit/widgets/text-editor-controls/button";

export default {
    title: "Module / _COMMON / edit / Widgets / Sidebar / Text Editor Controls"
}
interface Args {
    kind: Kind,
    active: false

}

const DEFAULT_ARGS:Args = {
    kind: "h1",
    active: false
}

export const Button = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <text-editor-controls-button ${argsToAttrs(props)}></text-editor-controls-button>
    `;
}

Button.args = DEFAULT_ARGS;
Button.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ['h1', 'h2', 'p1', 'p2', 'bold', 'italic', 'underline', 'align-left', 'align-center', 'align-right', 'color', 'highlight-color', 'box-color', 'indent', 'outdent']
        }
    },
}
