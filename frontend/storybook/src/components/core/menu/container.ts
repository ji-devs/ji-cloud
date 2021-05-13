import {argsToAttrs} from "@utils/attributes";
import {mapToString} from "@utils/array";
import "@elements/core/menu/container";
import "@elements/core/menu/menu-line";

export default {
    title: "Core / Menu"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Container = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    
    return `
    <menu-container ${argsToAttrs(props)}>
    ${
        mapToString(
            [   "", 
                "copy", 
                "delete", 
                "duplicate", 
                "edit", 
                "move-down", 
                "move-up", 
                "paste", 
                "print", 
                "reuse"
            ],
            kind => `<menu-line icon="${kind}"></menu-line>`
        )
        }
    </menu-container>
    `
}

Container.args = DEFAULT_ARGS;
