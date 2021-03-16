
import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/module/memory/edit/steps/sections/sidebar/step2/container";
import {Option} from "./option"; 
import {MODE} from "@elements/module/memory/_common/types.ts";
import {ThemeKind, ThemeKinds, ThemeControl} from "~/components/module/_common/theme";
export default {
    title: "Module / Memory / Edit / Steps / Sections / Sidebar / Step2"
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
    return `<step2-sidebar-container slot="content" ${argsToAttrs(props)}>
    ${mapToString(ThemeKinds, opt_theme => Option({
        theme: opt_theme,
        state: opt_theme === theme ? "selected" : "idle"
    }))}
    </step2-sidebar-container>`;
}

Container.args= DEFAULT_ARGS;

Container.argTypes = {
    theme: ThemeControl
}
