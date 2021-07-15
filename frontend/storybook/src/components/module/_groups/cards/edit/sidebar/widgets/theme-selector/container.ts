import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {ThemeId, ThemeIds, ThemeControl} from "~/components/module/_common/theme";
import {Option} from "./option"; 
import "@elements/module/_common/edit/widgets/theme-selector/container";
export default {
    title: "Module / _GROUPS / Cards / Edit / Sidebar / Widgets / Theme Selector"
}

interface Args {
    selected: ThemeId,
    jig: ThemeId,
}

const DEFAULT_ARGS:Args = {
    selected: "chalkboard",
    jig: "happy-brush",
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {selected, jig} = props;
    return `<theme-selector ${argsToAttrs(props)}>
    ${mapToString(ThemeIds, opt_theme => Option({
        theme: opt_theme,
        state: opt_theme === jig ? "jig"
        : opt_theme === selected ? "selected" 
        : "idle"
    }))}
    </theme-selector>`;
}

Container.args= DEFAULT_ARGS;

Container.argTypes = {
    selected: ThemeControl,
    jig: ThemeControl,
}
