import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/user/register/widgets/password-strength";
import { Strength } from "@elements/entry/user/register/widgets/password-strength";

export default {
    title: "Entry / User / Register / Widgets ",
};
interface Args {
    strength: Strength;
    width: number;
}

const DEFAULT_ARGS: Args = {
    strength: "average",
    width: 300,
};

export const PasswordStrength = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    const { width, ...psProps } = props;

    return `
        <div style="width: ${width}px">
            <password-strength ${argsToAttrs(psProps)} ></password-strength>
        </div>
    `;
};

PasswordStrength.args = DEFAULT_ARGS;

PasswordStrength.argTypes = {
    strength: {
        control: {
            type: "inline-radio",
            options: ["none", "weak", "average", "strong"],
        },
    },
};
