import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/publish";
import "@elements/entry/jig/edit/sidebar/module/menu";
import "@elements/core/menu/kebab";
import "@elements/core/menu/menu-line";

export default {
    title: "Entry / Jig / Edit / Sidebar"
}

interface Args {
    selected: boolean,
    published: boolean,
    collapsed: boolean,
    menuOpen: boolean,
    showAdvancedMenu: boolean,
}

const DEFAULT_ARGS:Args = {
    selected: true,
    published: false,
    collapsed: false,
    menuOpen: false,
    showAdvancedMenu: false,
}

type InternalExtra = {
    slot?: string,
}

export const Publish = (props?:Partial<Args> & InternalExtra) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-edit-sidebar-publish ${argsToAttrs(props)}>
            ${renderMenu(props.menuOpen, props.showAdvancedMenu)} 
        </jig-edit-sidebar-publish>
    `;
}

function renderMenu(visible: boolean, showAdvanced:boolean) {
    return `
        <menu-kebab ${visible && "visible"} slot="menu">
            <jig-edit-sidebar-module-menu slot="menu-content" ${showAdvanced && "advanced"}>
                <menu-line slot="lines" icon="edit"></menu-line>
                <menu-line slot="advanced" icon="copy" customLabel="Copy to another JIG"></menu-line>
                <menu-line slot="advanced" icon="copy" customLabel="Paste from another JIG"></menu-line>
            </jig-edit-sidebar-module-menu>
        </menu-kebab>
    `;
}

Publish.args = DEFAULT_ARGS;
