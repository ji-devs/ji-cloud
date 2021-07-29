import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/profile/reset-password-popup";
import "@elements/core/buttons/rectangle";
import "@elements/core/buttons/empty";

export default {
    title: 'Entry / User / Profile',
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const OptionsPopup = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <user-profile-reset-password-popup ${argsToAttrs(props)}>
            <button-empty slot="close">×</button-empty>
            <input-text slot="inputs" label="Current password" placeholder="Type your password"></input-text>
            <button-rect type="text" slot="inputs">Forgot your password?</button-rect>
            <input-text slot="inputs" label="New password" placeholder="Type your password"></input-text>
            <input-text slot="inputs" label="Retype new password" placeholder="Type your password"></input-text>
            <button-rect type="text" slot="cancel">Cancel</button-rect>
            <button-rect slot="save" color="blue">Save</button-rect>
        </user-profile-reset-password-popup>
    `
}

OptionsPopup.args = DEFAULT_ARGS;
