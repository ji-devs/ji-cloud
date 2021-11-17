import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/widgets/theme-selector/apply-popup";

export default {
    title: "Module / _GROUPS / Design / Edit / Sidebar / Widgets / Theme Selector",
};

interface Args {
}

const DEFAULT_ARGS: Args = {
};

export const ApplyPopup = (props?: Partial<Args> & { content?: string }) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <theme-selector-apply-popup ${argsToAttrs(props)}>
            <button-rect slot="actions" kind="text" color="blue">Apply to cover only</button-rect>
            <button-rect slot="actions" kind="filled" color="blue">Apply to JIG</button-rect>
        </theme-selector-apply-popup>
    `;
};

ApplyPopup.args = DEFAULT_ARGS;

