import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/footer";
export default {
    title: "Module / _common"
}

const STR_CONTINUE = "Continue";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Footer = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `<module-footer slot="footer">

        <button-rect color="grey" size="small" iconAfter="arrow" slot="btn">${STR_CONTINUE}</button-rect>
    </module-footer>`
}

Footer.Args = DEFAULT_ARGS;
