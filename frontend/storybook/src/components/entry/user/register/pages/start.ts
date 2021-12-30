import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/user/register/pages/start";
import "@elements/entry/user/register/footer/login";
import "@elements/core/buttons/rectangle";
import "@elements/entry/user/_common/buttons/google";

import { Strength as PasswordStrength } from "@elements/entry/user/register/widgets/password-strength";

export default {
    title: "Entry / User / Register / Pages",
};

const STR_SUBMIT = "Submit";
const STR_EMAIL_LABEL = "Email";
const STR_PASSWORD_LABEL = "Create Password";
const STR_PASSWORD_PLACEHOLDER = "********";
const STR_CONTINUE = "Continue";

interface Args {
    passwordStrength: PasswordStrength;
}

const DEFAULT_ARGS: Args = {
    passwordStrength: "none",
};

export const Start = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    const { passwordStrength } = props;

    return `
        <page-register-start passwordStrength="${passwordStrength}">
            <button-google slot="google"></button-google>
            <input-wrapper slot="email" label="${STR_EMAIL_LABEL}">
                <input>
            </input-wrapper>
            <input-password slot="password" label="${STR_PASSWORD_LABEL}" placeholder="${STR_PASSWORD_PLACEHOLDER}"></input-password>
            <button-rect slot="submit" color="red" size="medium" IconAfter="arrow">${STR_CONTINUE}</button-rect>
            <footer-register-login slot="footer"></footer-register-login>
        </page-register-start>
    `;
};

Start.args = DEFAULT_ARGS;

Start.argTypes = {
    passwordStrength: {
        control: {
            type: "inline-radio",
            options: ["none", "weak", "average", "strong"],
        },
    },
};
