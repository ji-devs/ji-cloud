import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {ThemeKind, ThemeKinds, ThemeControl} from "~/components/module/_common/edit/theme";
import {Option} from "./option"; 
import "@elements/module/_groups/cards/edit/sidebar/widgets/theme-selector/container";

export default {
    title: "Module / _GROUPS / Cards / Edit / Sidebar / Widgets / Theme Selector"
}

interface Args {
    theme: ThemeKind
}

const DEFAULT_ARGS:Args = {
    theme: "chalkboard"
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {theme} = props;
    return `<theme-selector-cards ${argsToAttrs(props)}>
    ${mapToString(ThemeKinds, opt_theme => Option({
        theme: opt_theme,
        state: opt_theme === theme ? "selected" : "idle"
    }))}
    </theme-selector-cards>`;
}

Container.args= DEFAULT_ARGS;

Container.argTypes = {
    theme: ThemeControl
}
