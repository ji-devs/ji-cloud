import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/edit/sidebar/settings/jig-settings-themes";

export default {
    title: "Entry / Jig / Edit / Sidebar / Settings"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const SettingsThemes = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-settings-themes ${argsToAttrs(props)}>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
            <theme-selector-design-option></theme-selector-design-option>
        </jig-settings-themes>
    `;
}
SettingsThemes.args = DEFAULT_ARGS;

