import "@elements/module/_common/edit/widgets/color-select/color-select";
import "@elements/module/_common/edit/widgets/color-select/color-select-section";
import "@elements/module/_common/edit/widgets/color-select/color-select-item";
import "@elements/core/inputs/primitives/color";
import "@elements/core/buttons/rectangle";
import "@elements/core/buttons/icon";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _COMMON /  edit /Widgets / Sidebar / Color Select",
};

interface Args {
    label: string;
}

const DEFAULT_ARGS: Args = {
    label: "Select color for the background",
};

const generalColorList = [
    "#00000000",
    "#ffffffff",
    "#fffcc7ff",
    "#fff445ff",
    "#feae2aff",
    "#f34826ff",
    "#ff0303ff",
    "#fdcdf1ff",
    "#f74ac8ff",
    "#da0f63ff",
    "#9517acff",
    "#7a28fbff",
    "#b0c7faff",
    "#2d9bf0ff",
    "#414cb3ff",
    "#09168dff",
    "#22bed9ff",
    "#1aa09dff",
    "#077472ff",
    "#8fd150ff",
    "#cfe741ff",
    "#cececeff",
    "#808080ff",
    "#1a1a1aff",
];

const themeColorList = ["#dce9f5", "#e6e6e6", "#808080", "#1a1a1a"];

const userColorList = ["#dce9f5", "#e6e6e6", "#808080", "#1a1a1a"];

export const colorSelect = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <color-select ${argsToAttrs(props)}>
            <color-select-section slot="sections" label="General colors">
                ${generalColorList.map(
                    (color) =>
                        `<color-select-item slot="items" color="${color}"></color-select-item>`
                )}
            </color-select-section>
            <color-select-section slot="sections" label="Theme colors">
                ${themeColorList.map(
                    (color) =>
                        `<color-select-item slot="items" color="${color}"></color-select-item>`
                )}
            </color-select-section>
            <color-select-section slot="sections" label="My colors">
                ${userColorList.map(
                    (color) =>
                        `<color-select-item slot="items" color="${color}" deletable>
                        <button-icon slot="delete-button" icon="circle-x-blue"></button-icon>
                    </color-select-item>`
                )}
            </color-select-section>

            <input-color slot="add-color">
                <button-rect kind="text">+ Add color</button-rect>
            </input-color>
        </color-select>
    `;
};

colorSelect.args = DEFAULT_ARGS;
