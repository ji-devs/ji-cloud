import "@elements/core/menu/menu-tabs/menu-tabs";
import "@elements/core/menu/menu-tabs/menu-tab";
import "@elements/core/menu/menu-tabs/menu-tab-with-title";
import { argsToAttrs } from "@utils/attributes";
export default {
    title: "Core / Menu / Menu Tab"
}

interface Args {
    small: boolean
}

const DEFAULT_ARGS:Args = {
    small: true 
}

export const MenuTabs = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {small, ...menuProps} = props;

    return `
        <menu-tabs ${argsToAttrs(menuProps)}>
            <menu-tab-with-title slot="tabs" kind="text" ${small ? "small" : ""}></menu-tab-with-title>
            <menu-tab-with-title slot="tabs" kind="image" active></menu-tab-with-title>
            <menu-tab-with-title slot="tabs" kind="audio" ${small ? "small" : ""}></menu-tab-with-title>
            <menu-tab-with-title slot="tabs" kind="color" ${small ? "small" : ""}></menu-tab-with-title>
            <div slot="body">
                Tab body
            </div>
        </menu-tabs>
    `;
}

MenuTabs.args = DEFAULT_ARGS;
