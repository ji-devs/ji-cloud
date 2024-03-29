import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/user/register-complete/pages/landing";
import "@elements/core/buttons/rectangle";

export default {
    title: "Entry / User / Register / Pages",
};

const STR_BUTTON = "Go to JI home";

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Complete = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <page-register-complete>
            <button-rect slot="button" color="red">${STR_BUTTON}</button-rect>
        </page-register-complete>
    `;
};

Complete.args = DEFAULT_ARGS;
