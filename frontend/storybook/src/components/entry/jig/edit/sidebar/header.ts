
import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/sidebar";
import "@elements/entry/jig/edit/sidebar/header";

const STR_MY_JIGS = "My JIGs";
export default {
    title: "Entry / Jig / Edit / Sidebar"
}

interface Args {
    collapsed: boolean,
    isModulePage: boolean,
}

const DEFAULT_ARGS:Args = {
    collapsed: false,
    isModulePage: false,
}

export const Header = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-edit-sidebar-header slot="header" ${argsToAttrs(props)}>
            <img-ui slot="close" path="entry/jig/collapse.svg"></img-ui>
            <button-text slot="gallery" color="blue" weight="medium">${STR_MY_JIGS}</button-text>
            <img-ui slot="settings" path="entry/jig/settings.svg"></img-ui>
            <img-ui slot="modules" path="entry/jig/modules.svg"></img-ui>
            <input-text-pencil slot="input" placeholder="My JIG’s name"></input-text-pencil>
        </jig-edit-sidebar-header>
    `;
}
Header.args = DEFAULT_ARGS;
