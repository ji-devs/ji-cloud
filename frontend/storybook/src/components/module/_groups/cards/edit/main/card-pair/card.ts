import {argsToAttrs} from "@utils/attributes";
import {ThemeKind, ThemeControl} from "~/components/module/_common/theme";
import "@elements/core/inputs/text-content";
import {Ji as MockJiImage} from "~/components/core/images/ji";
import "@elements/core/inputs/textarea-content";
import "@elements/module/_groups/cards/edit/main/card-pair/card";
import {Mode} from "@elements/module/_groups/cards/helpers";

export default {
    title: "Module / _GROUPS / Cards / edit / Main"
}

type CONTENT_MODE = "text" | "image" | "image-empty";
type IO_MODE = "edit" | "preview" | "static";

export interface Args {
    ioMode: IO_MODE,
    contentMode: CONTENT_MODE,
    theme: ThemeKind,
    dragOver: boolean,
    inverted: boolean,
    mode: Mode,
}

const DEFAULT_ARGS:Args = {
    ioMode: "preview",
    contentMode: "text",
    theme: "chalkboard",
    dragOver: false,
    inverted: false,
    mode: "lettering"
}

export const Card= (props?:Partial<Args> & {slot?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {slot, contentMode, ioMode, ...cardProps} = props;

    Object.assign(cardProps, {
        flippable: ioMode === "preview",
        editing: ioMode === "edit" 
    });

    /*
    if(cardProps.theme === "") {
        delete cardProps["theme"];
    }
    */
    return `
    <main-card ${argsToAttrs(cardProps)} ${slot ? `slot="${slot}"` : ""}>
    ${getContent(contentMode, ioMode)}
    </main-card>`
}

function getContent(contentMode: CONTENT_MODE, ioMode: IO_MODE) {
    const editing = ioMode === "edit"; 
    if(contentMode === "text") {
        const value = "hello";
        return `<input-textarea-content value="${value}" ${editing}></input-textarea-content>`;
    } else if(contentMode === "image") {
        return MockJiImage({size: "thumb"})
    } else if(contentMode === "image-empty") {
        return `<img-ui path="core/_common/image-empty.svg"></img-ui>`
    }
}
Card.args = DEFAULT_ARGS;
Card.argTypes = {
    ioMode: {
        control: {
            type: 'inline-radio',
            options: ["edit", "preview"]
        }
    },
    contentMode: {
        control: {
            type: 'inline-radio',
            options: ["text", "image", "image-empty"]
        }
    },
    theme: ThemeControl
}
