import "@elements/core/menu/menu-tabs/menu-tab";
import "@elements/core/menu/menu-tabs/menu-tab-title";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Core / Menu / Menu Tab",
};

interface Args {
    active: boolean;
}

const DEFAULT_ARGS: Args = {
    active: true,
};

export const MenuTab = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <menu-tab ${argsToAttrs(props)}>
            <menu-tab-title kind="text" ${
                props.active ? "active" : ""
            }></menu-tab-title>
        </menu-tab>
    `;
};

MenuTab.args = DEFAULT_ARGS;
