import "@elements/widgets/color-select/color-select";
import "@elements/widgets/color-select/color-select-item";
import "@elements/core/inputs/color";
import "@elements/core/buttons/text";

export default {
    title: "Widgets / Color Select"
}

interface Args {
}

const DEFAULT_ARGS: Args = {
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
    "#fff",
    "#fff",
    "#fff",
    "#fff",
    "#fff",
    "#fff",
]


export const colorSelect = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <color-select label="Select background">
            ${colorList.map(color => (
                `<color-select-item slot="items" color="${color}"></color-select-item>`
            ))}

            <input-color slot="add-color">
                <button-text>+ Add color</button-text>
            </input-color>
        </color-select>
    `;
}

colorSelect.args = DEFAULT_ARGS;
