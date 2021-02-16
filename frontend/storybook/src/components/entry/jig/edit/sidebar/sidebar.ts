import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/sidebar";
import "@elements/entry/jig/edit/sidebar/header";
import {mapToString, arrayIndex} from "@utils/array";
import {Module} from "./module";

export default {
    title: "Entry / Jig / Edit / Sidebar"
}

interface Args {
    nModules: number
}

const DEFAULT_ARGS:Args = {
    nModules: 10
}

export const WithModules = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    
    const {nModules} = props;

    return `
        <jig-edit-sidebar>
        <jig-edit-sidebar-header slot="header"> </jig-edit-sidebar-header>
        ${mapToString(arrayIndex(nModules), index => {
            return Module({
                module: index === 0 ? "cover" : "memory",
                rawIndex: index,
                menuOpen: false,
                slot: index === 0 ? "cover-module" : "modules",
                selected: index === 1,
                makeDemoRoomAtTop: false,
                lastBottomDecoration: index === nModules-1
            });
        })}
        </jig-edit-sidebar>
    `;
}

WithModules.args = DEFAULT_ARGS;
