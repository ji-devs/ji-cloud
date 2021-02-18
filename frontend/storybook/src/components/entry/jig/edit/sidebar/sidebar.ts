import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/sidebar";
import "@elements/entry/jig/edit/sidebar/header";
import {mapToString, arrayIndex} from "@utils/array";
import {Module} from "./module";

export default {
    title: "Entry / Jig / Edit / Sidebar"
}

interface Args {
    nModules: number,
    menuIndex: number
}

const DEFAULT_ARGS:Args = {
    nModules: 10,
    menuIndex: 1
}

export const WithModules = (props?:Partial<Args> & {slot?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    
    const {slot, menuIndex, nModules} = props;

    return `
        <jig-edit-sidebar ${slot && `slot="${slot}"`}>
        <jig-edit-sidebar-header slot="header"> </jig-edit-sidebar-header>
        ${mapToString(arrayIndex(nModules), index => {
            return Module({
                module: index === 0 ? "cover" : "memory",
                rawIndex: index,
                menuOpen: index === menuIndex,
                slot: index === 0 ? "cover-module" : "modules",
                selected: index === 1,
                makeDemoRoomAtTop: false,
                lastModule: index === nModules-1,
                showAdd: index !== nModules-1,

            });
        })}
        </jig-edit-sidebar>
    `;
}

WithModules.args = DEFAULT_ARGS;
