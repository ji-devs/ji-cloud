import "@elements/core/menu/menu-tabs/menu-tabs";
import "@elements/core/menu/menu-tabs/menu-tab";
import "@elements/core/menu/menu-tabs/menu-tab-title";
import "@elements/core/buttons/circle";
import { argsToAttrs } from "@utils/attributes";
export default {
    title: "Core / Menu / Menu Tab"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const MenuTabs = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <menu-tabs ${argsToAttrs(props)}>
            <menu-tab slot="tabs">
                <menu-tab-title kind="text"></menu-tab-title>
            </menu-tab>
            <menu-tab slot="tabs" active>
                <menu-tab-title kind="image"></menu-tab-title>
            </menu-tab>
            <menu-tab slot="tabs">
                <menu-tab-title kind="audio"></menu-tab-title>
            </menu-tab>
            <div slot="body">
                Tab body
            </div>
        </menu-tabs>
    `;
}

MenuTabs.args = DEFAULT_ARGS;
