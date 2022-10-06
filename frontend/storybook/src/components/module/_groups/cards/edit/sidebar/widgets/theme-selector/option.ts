import { argsToAttrs } from "@utils/attributes";
import { mapToString, arrayCount } from "@utils/array";
import {
    ThemeId,
    ThemeIds,
    ThemeControl,
} from "~/components/module/_common/theme";
import "@elements/core/menu/kebab";
import "@elements/core/menu/menu-line";
import "@elements/module/_common/edit/widgets/theme-selector/option";
import { STATE } from "@elements/module/_common/edit/widgets/theme-selector/option";

export default {
    title: "Module / _GROUPS / Cards / Edit / Sidebar / Widgets / Theme Selector",
};

interface Args {
    theme: ThemeId;
    state: STATE;
    optionType?: String,
}

const DEFAULT_ARGS: Args = {
    theme: "chalkboard",
    state: "idle",
    optionType: "card",
};

export const Option = (props?: Partial<Args> & { content?: string }) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    return `
    <theme-selector-cards-option ${argsToAttrs(props)}>
       <menu-line slot="menu" icon="set-jig-theme"></menu-line>
    </theme-selector-cards-option>`;
};

Option.args = DEFAULT_ARGS;

Option.argTypes = {
    state: {
        control: {
            type: "inline-radio",
            options: ["idle", "hover", "selected"],
        },
    },
    theme: ThemeControl,
};
