import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/admin/sidebar";

export default {
    title: "Entry/Admin/Sidebar"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Sidebar = (props?:Partial<Args>) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<admin-sidebar ${argsToAttrs(props)}></admin-sidebar>`;
}

Sidebar.args = DEFAULT_ARGS;