import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/publish";
import "@elements/entry/jig/edit/sidebar/module/window";
import { ModuleState } from "@elements/entry/jig/edit/sidebar/module/window";
import { ModuleKind, moduleKinds } from "@elements/module/_common/types";

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
    activeModuleKind: "drag-drop",
    publishedThumbnail: "",
}


export const ModuleWindow = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <div style="margin: 50px;">
            <jig-edit-sidebar-module-window ${argsToAttrs(props)}>
                <img-module-screenshot
                    jigId="f9ada00c-ca99-11eb-8e32-63cc69e065b5"
                    moduleId="4f90affc-f54a-11eb-be74-0b241af9476c"
                    fallbackKind="poster"
                ></img-module-screenshot>
            </jig-edit-sidebar-module-window>
        </div>
    `;
}

ModuleWindow.argTypes = {
    state: {
        control: {
            type: 'inline-radio',
            // options: ["empty", "draft", "active", "complete", "published"],
            options: ["empty", "active", "thumbnail"],
        }
    },
    activeModuleKind: {
        control: {
            type: 'inline-radio',
            options: moduleKinds,
        }
    }
}
