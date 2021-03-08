import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/admin/sidebar";
import {SECTION} from "@elements/entry/admin/sidebar";

export default {
    title: "Entry/Admin/Sidebar"
}

interface Args {
    closed: boolean,
    imageLocked: boolean,
    jigLocked: boolean,
    categoryLocked: boolean,
    localeLocked: boolean,
    section: SECTION,
}

const DEFAULT_ARGS:Args = {
    closed: false,
    imageLocked: false,
    jigLocked: true,
    categoryLocked: false,
    localeLocked: false,
    section: ""
}

export const Sidebar = (props?:Partial<Args>) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<admin-sidebar ${argsToAttrs(props)}></admin-sidebar>`;
}

Sidebar.args = DEFAULT_ARGS;
Sidebar.argTypes = {
    section: {
        control: {
            type: 'inline-radio',
            options: ["", "image-add", "image-search", "jig", "category", "locale"]
        }
    }
}
