import "@elements/module/_common/edit/widgets/color-select/color-select-section";
import "@elements/module/_common/edit/widgets/color-select/color-select-item";
import "@elements/core/inputs/primitives/color";
import "@elements/core/buttons/text";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _COMMON /  edit /Widgets / Sidebar / Color Select"
}

interface Args {
    label: string;
}

const DEFAULT_ARGS: Args = {
    label: "General colors",
}

const colorList = [
    "transparent",
    "#fff",
    "#fff445",
    "#fac72d",
    "#feae2a",
    "#f34826",
    "#fb178d",
    "#da0f63",
    "#f74ac8",
    "#9517ac",
    "#7a28fb",
    "#414cb3",
    "#2d9bf0",
    "#22cdd4",
    "#18a789",
    "#8fd150",
    "#cfe741",
    "#bbccf8",
    "#dce9f5",
    "#e6e6e6",
    "#808080",
    "#1a1a1a",
    "#fff",
    "#fff",
]


export const colorSelectSection = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <color-select-section  ${argsToAttrs(props)}>
            ${colorList.map(color => (
                `<color-select-item slot="items" color="${color}"></color-select-item>`
            ))}
        </color-select-section>
    `;
}

colorSelectSection.args = DEFAULT_ARGS;
