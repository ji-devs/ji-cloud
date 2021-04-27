import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/play/card/card";
import "@elements/module/memory/play/card/text";
import {mapToString, arrayIndex} from "@utils/array";
import {ThemeKind, ThemeControl} from "~/components/module/_common/theme";
import {Ji as MockJiImage} from "~/components/core/images/ji";

type CONTENT_MODE = "text" | "image";

export default {
    title: "Module / Memory / Play"
}

interface Args {
    scale: number,
    translateX: number,
    translateY: number,
    transform: boolean,
    theme: ThemeKind,
    flipped: boolean,
    contentMode: CONTENT_MODE,
}

const DEFAULT_ARGS:Args = {
    scale: 1,
    translateX: 0,
    translateY: 0,
    transform: false,
    theme: "chalkboard",
    flipped: true,
    contentMode: "image",
}

export const Card = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {contentMode, ...cardProps} = props;

    return `
    <play-card ${argsToAttrs(cardProps)} >
        ${getContent(contentMode)}
    </play-card>`;
}

function getContent(contentMode: CONTENT_MODE) {
    if(contentMode === "text") {
        const value = "hello";
        return `<card-text value="${value}"></card-text>`;
    } else if(contentMode === "image") {
        return MockJiImage({size: "thumb"})
    } 
}

Card.args = DEFAULT_ARGS;
Card.argTypes = {
    contentMode: {
        control: {
            type: 'inline-radio',
            options: ["text", "image", "image-empty"]
        }
    },
    theme: ThemeControl
}
