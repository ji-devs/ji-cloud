import {argsToAttrs} from "@utils/attributes";
import "@elements/core/menu/menu-line";
import {IconKind} from "@elements/core/menu/menu-line";

export default {
    title: "Core / Menu"
}

interface Args {
    icon: IconKind,
    customLabel: string
}

const DEFAULT_ARGS:Args = {
    icon: "copy",
    customLabel: ""
}

export const MenuLine = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    
    return `
        <menu-line ${argsToAttrs(props)} />
    `
}

MenuLine.args = DEFAULT_ARGS;

MenuLine.argTypes = {
    icon: {
        control: {
            type: 'inline-radio',
            options: [
                "", 
                "copy",
                "paste",
                "delete",
                "duplicate",
                "edit",
                "move-down",
                "move-up",
                "print",
                "reuse",
                //all stickers
                "move-forward",
                "move-backward",
                "flip-horizontal",
                "flip-vertical",
                //bg only
                "change-background-color",
                "change-background-image",
                "remove-background-image",
                "remove-overlay",
                //image only
                "crop",
                "remove-white",
                "make-background",
                "play",
                "record-sound",
                "upload-sound",
                //module publish
                "use-content-as",
            ]
        }
    },
}


