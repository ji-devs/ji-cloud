import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/_common/footer";
export default {
    title: "Module / Memory / Edit / Steps"
}

const STR_CONTINUE = "Continue";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Footer = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `<steps-footer slot="footer">

        <button-rect color="grey" size="small" iconAfter="arrow" slot="btn">${STR_CONTINUE}</button-rect>
    </steps-footer>`
}

Footer.Args = DEFAULT_ARGS;
