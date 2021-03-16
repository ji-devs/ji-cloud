import { argsToAttrs } from "@utils/attributes";
import "@elements/module/cover/sidebar/sidebar-jig-settings";

export default {
    title: "Module / Cover / Sidebar"
}

interface Args {
    value: number,
    min: number,
    max: number,
}

const DEFAULT_ARGS: Args = {
    value: 3,
    min: -2,
    max: 4,
}

export const SidebarJigSettings = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <sidebar-jig-settings ${argsToAttrs(props)}>
            <label slot="direction-options">
                <input type="radio" name="direction" value="ltr">
                Left to right
            </label>
            <label slot="direction-options">
                <input type="radio" name="direction" value="rtl">
                Right to left
            </label>
        </sidebar-jig-settings>
    `;
}

SidebarJigSettings.args = DEFAULT_ARGS;
