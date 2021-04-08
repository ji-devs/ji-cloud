
import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/sidebar";
import "@elements/entry/jig/edit/sidebar/header";

const STR_MY_JIGS = "See my JIGs";
export default {
    title: "Entry / Jig / Edit / Sidebar"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Header = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
    <jig-edit-sidebar-header slot="header">
        <button-icon slot="close" icon="x"></button-icon>
        <button-text slot="gallery" color="blue" weight="medium">${STR_MY_JIGS}</button-text>
        <input-text-pencil slot="input"></input-text-pencil>
    </jig-edit-sidebar-header>
    `;
}
Header.args = DEFAULT_ARGS;
