import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/choose/card";
import {MODE} from "@elements/module/memory/edit/choose/card";

export default {
    title: "Module / Memory / Edit / Choose"
}

interface Args {
    mode:MODE
}

const DEFAULT_ARGS:Args = {
    mode: "duplicate"
}

export const Card = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    
    return `
    <choose-card ${argsToAttrs(props)}></choose-card>
    `;
}

Card.Args = DEFAULT_ARGS;
Card.argTypes = {
    mode: {
        control: {
            type: 'inline-radio',
            options: ["duplicate", "words-images", "begins", "lettering"]
        }
    }
}
