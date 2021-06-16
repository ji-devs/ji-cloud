import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {ThemeKind, ThemeKinds, ThemeControl} from "~/components/module/_common/theme";
import {Option} from "./option"; 
import "@elements/module/_groups/design/edit/sidebar/widgets/theme-selector/container";

export default {
    title: "Module / _GROUPS / Design / Edit / Sidebar / Widgets / Theme Selector"
}



interface Args {
    selected: ThemeKind,
    jig: ThemeKind,
}

const DEFAULT_ARGS:Args = {
    selected: "chalkboard",
    jig: "happy-brush",
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {selected, jig} = props;
    return `<theme-selector-cards ${argsToAttrs(props)}>
    ${mapToString(ThemeKinds, opt_theme => Option({
        theme: opt_theme,
        state: opt_theme === jig ? "jig"
        : opt_theme === selected ? "selected" 
        : "idle"
    }))}
    </theme-selector-cards>`;
}

Container.args= DEFAULT_ARGS;

Container.argTypes = {
    selected: ThemeControl,
    jig: ThemeControl,
}