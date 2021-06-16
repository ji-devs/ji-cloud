import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import {ThemeKind, ThemeKinds, ThemeControl} from "~/components/module/_common/theme";
import "@elements/module/_groups/design/edit/sidebar/widgets/theme-selector/option";
import {STATE} from "@elements/module/_groups/design/edit/sidebar/widgets/theme-selector/option";

export default {
    title: "Module / _GROUPS / Design / Edit / Sidebar / Widgets / Theme Selector"
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
    return `
    <theme-selector-design-option ${argsToAttrs(props)}>
       <menu-line slot="menu" icon="set-jig-theme"></menu-line>
    </theme-selector-design-option>`;
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
