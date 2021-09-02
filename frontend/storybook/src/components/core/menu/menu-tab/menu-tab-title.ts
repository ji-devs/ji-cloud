import "@elements/core/menu/menu-tabs/menu-tab-title";
import "@elements/core/buttons/circle";
import { argsToAttrs } from "@utils/attributes";
import { TabKind } from "@elements/core/menu/menu-tabs/menu-tab-title";
export default {
    title: "Core / Menu / Menu Tab"
}

interface Args {
    kind: TabKind,
    active: boolean,
}

const DEFAULT_ARGS: Args = {
    kind: 'text',
    active: false,
}

export const MenuTabTitle = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <menu-tab-title ${argsToAttrs(props)}></menu-tab-title>
    `;
}

MenuTabTitle.args = DEFAULT_ARGS;

MenuTabTitle.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ['theme', 'background-image', 'background-color', 'color', 'overlay', 'text', 'image', 'audio', 'video', 'select', 'play-settings', 'instructions']
        }
    }
}
