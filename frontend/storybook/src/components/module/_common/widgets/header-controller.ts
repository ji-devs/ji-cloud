import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/widgets/header-controller";

export default {
    title: "Module / _common / Widgets"
}


interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const HeaderController = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `<module-header-controller slot="controller" ${argsToAttrs(props)}>
    </module-header-controller>`
}

HeaderController.Args = DEFAULT_ARGS;
