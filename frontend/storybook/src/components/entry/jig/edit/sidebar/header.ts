
import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/sidebar";
import "@elements/entry/jig/edit/sidebar/header";

export default {
    title: "Entry / Jig / Edit / Sidebar"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Header = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<jig-edit-sidebar-header></jig-edit-sidebar-header>`;
}

Header.args = DEFAULT_ARGS;
