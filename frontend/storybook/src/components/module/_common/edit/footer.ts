import { argsToAttrs } from "@utils/attributes";
import "@elements/module/_common/edit/footer";
export default {
    title: "Module / _common / edit",
};

const STR_CONTINUE = "Continue";

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Footer = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `<module-footer slot="footer">

        <button-rect disabled size="small" iconAfter="arrow" slot="btn">${STR_CONTINUE}</button-rect>
    </module-footer>`;
};

Footer.args = DEFAULT_ARGS;
