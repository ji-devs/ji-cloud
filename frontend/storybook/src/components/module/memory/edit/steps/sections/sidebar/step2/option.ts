import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/module/memory/edit/steps/sections/sidebar/step2/option";
import {STATE} from "@elements/module/memory/edit/steps/sections/sidebar/step2/option";
import {MODE} from "@elements/module/memory/_common/types";
import {ThemeKind, ThemeKinds, ThemeControl} from "~/components/module/_common/theme";
export default {
    title: "Module / Memory / Edit / Steps / Sections / Sidebar / Step2"
}

interface Args {
    theme: ThemeKind,
    state: STATE,
}

const DEFAULT_ARGS:Args = {
    theme: "chalkboard",
    state: "idle"
}

export const Option = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    return `<step2-sidebar-option ${argsToAttrs(props)}></step2-sidebar-option>`;
}

Option.args= DEFAULT_ARGS;

Option.argTypes = {
  state: {
    control: {
      type: 'inline-radio',
      options: ["idle", "hover", "selected"]
    }
  },
    theme: ThemeControl
}
