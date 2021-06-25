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
    slot?: string,
    backSideContent: CONTENT_MODE | "none",
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
    size: "memory",
    mode: "lettering",
    side: "left",
    backSideContent: "none"
}

export const Card = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {contentMode, backSideContent, ...cardProps} = props;

    if(backSideContent !== "none") {
        (cardProps as any).doubleSided = true;
    }
    return `
    <play-card ${argsToAttrs(cardProps)} >
        ${getContent(contentMode)}
        ${backSideContent !== "none" ? getContent(backSideContent, "backSideContent") : ``}
    </play-card>`;
}

function getContent(contentMode: CONTENT_MODE, slot?: string) {
    const slotAttr = slot ? `slot="${slot}"` : "";
    if(contentMode === "text") {
        const value = "hello";
        return `<card-text value="${value}" ${slotAttr}></card-text>`;
    } else if(contentMode === "image") {
        return MockJiImage({size: "thumb", slot})
    } else if(contentMode === "image-empty") {
        return `<img-ui path="core/_common/image-empty.svg" ${slotAttr}></img-ui>`
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
    backSideContent: {
        control: {
            type: 'inline-radio',
            options: ["none", "text", "image", "image-empty"]
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
