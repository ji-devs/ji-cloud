import {argsToAttrs} from "@utils/attributes";
import "@elements/core/menu/menu-tab";

export default {
    title: "Core / Menu"
}

interface Args {
    contents: string
}

const DEFAULT_ARGS:Args = {
    contents: "hello"
}

export const MenuTab = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {contents} = props

    return `<menu-tab ${argsToAttrs(props)}>${contents}</menu-tab>`;
}

MenuTab.args = DEFAULT_ARGS;