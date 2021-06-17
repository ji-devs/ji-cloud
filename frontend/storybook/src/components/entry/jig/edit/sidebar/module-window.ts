import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/publish";
import "@elements/entry/jig/edit/sidebar/module/window";
import { ModuleState } from "@elements/entry/jig/edit/sidebar/module/window";
import { ModuleKind, moduleKinds } from "@elements/entry/jig/module-types";

export default {
    title: "Entry / Jig / Edit / Sidebar"
}

interface Args {
    state: ModuleState,
    activeModuleKind: ModuleKind,
    publishedThumbnail: string,
}

const DEFAULT_ARGS:Args = {
    state: "empty",
    activeModuleKind: "poster",
    publishedThumbnail: "",
}


export const ModuleWindow = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <div style="margin: 50px;">
            <jig-edit-sidebar-module-window ${argsToAttrs(props)}></jig-edit-sidebar-module-window>
        </div>
    `;
}

ModuleWindow.argTypes = {
    state: {
        control: {
            type: 'inline-radio',
            options: ["empty", "draft", "active", "complete", "published"],
        }
    },
    activeModuleKind: {
        control: {
            type: 'inline-radio',
            options: moduleKinds,
        }
    }
}
