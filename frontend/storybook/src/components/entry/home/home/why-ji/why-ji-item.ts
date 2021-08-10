import "@elements/entry/home/home/why-ji/home-why-ji-item";
import { Kind } from "@elements/entry/home/home/why-ji/home-why-ji-item";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: 'Entry / Home / Home / Why Ji',
}

interface ParagraphArgs {
    kind: Kind
}

const DEFAULT_ARGS: ParagraphArgs = {
    kind: "classroom",
}

export const WhyJiItem = (props?: ParagraphArgs) => {
    return `
        <home-why-ji ${argsToAttrs(props)}>
            ${Text({ contents: "Action text", color: "blue" })} 
        </home-why-ji>
    
    `
}

WhyJiItem.args = DEFAULT_ARGS;
WhyJiItem.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ['content', 'create', 'customize', 'community', 'classroom']
        }
    },
}
