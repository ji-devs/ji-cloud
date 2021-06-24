import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_groups/cards/play/card/card";
import "@elements/module/_groups/cards/play/card/text";
import {mapToString, arrayIndex} from "@utils/array";
import {ThemeKind, ThemeControl} from "~/components/module/_common/theme";
import {Ji as MockJiImage} from "~/components/core/images/ji";
import {Size} from "@elements/module/_groups/cards/play/card/card";
import {Mode, Side} from "@elements/module/_groups/cards/helpers";

type CONTENT_MODE = "text" | "image";

export default {
    title: "Module / _GROUPS / Cards / play"
}

export interface Args {
    scale: number,
    translateX: number,
    translateY: number,
    transform: boolean,
    theme: ThemeKind,
    flipped: boolean,
    flipOnHover: boolean,
    contentMode: CONTENT_MODE,
    size: Size,
    side: Side,
    mode: Mode,
    slot?: string
}

const DEFAULT_ARGS:Args = {
    scale: 1,
    translateX: 0,
    translateY: 0,
    transform: false,
    theme: "chalkboard",
    flipped: true,
    flipOnHover: false,
    contentMode: "text",
    size: "regular",
    mode: "lettering",
    side: "left"
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
    side: {
        control: {
            type: 'inline-radio',
            options: ["left", "right"]
        }
    },
    theme: ThemeControl
}
