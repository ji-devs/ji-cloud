import "@elements/entry/home/why-ji/home-why-ji-item";
import { Kind } from "@elements/entry/home/why-ji/home-why-ji-item";
import { Text } from "~/components/core/buttons/text";
import { PlainTextButton } from "~/components/entry/home/sections/plain-text-button";
import { Color, Size, Weight } from "@elements/core/buttons/text";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: 'Entry/ Home / Why Ji',
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
