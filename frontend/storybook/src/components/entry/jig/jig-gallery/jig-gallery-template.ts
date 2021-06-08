
import "@elements/entry/jig/gallery/template";
import { Kind } from "@elements/entry/jig/gallery/template";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Entry / Jig / Gallery"
}

interface Args {
    kind: Kind;
}

const DEFAULT_ARGS:Args = {
    kind: "vocabulary",
}

export const Template = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-gallery-template ${argsToAttrs(props)}></jig-gallery-template>
    `;
}

Template.args = DEFAULT_ARGS;
Template.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["vocabulary", "parsha"]
        }
    }
}
