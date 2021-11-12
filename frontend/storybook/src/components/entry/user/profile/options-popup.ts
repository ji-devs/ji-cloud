import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/user/profile/options-popup";
import "@elements/core/buttons/rectangle";
import "@elements/core/buttons/empty";

export default {
    title: "Entry / User / Profile",
};

interface Args {
    header: string;
    subheader: string;
}

const DEFAULT_ARGS: Args = {
    header: "Relevant Age Group",
    subheader: "Which age group are you interested in?",
};

export const OptionsPopup = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <user-profile-options-popup ${argsToAttrs(props)}>
            <button-empty slot="close">×</button-empty>
            <input-checkbox slot="options" label="something"></input-checkbox>
            <input-checkbox slot="options" label="something"></input-checkbox>
            <input-checkbox slot="options" label="something"></input-checkbox>
            <input-checkbox slot="options" label="something"></input-checkbox>
            <input-checkbox slot="options" label="something"></input-checkbox>
            <input-checkbox slot="options" label="something"></input-checkbox>
            <button-rect type  slot="cancel">Cancel</button-rect>
            <button-rect slot="save" color="blue">Save</button-rect>
        </user-profile-options-popup>
    `;
};

OptionsPopup.args = DEFAULT_ARGS;
