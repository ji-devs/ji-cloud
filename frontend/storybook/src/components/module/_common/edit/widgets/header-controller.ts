import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/edit/widgets/header-controller";

export default {
    title: "Module / _COMMON /  edit /Widgets"
}


interface Args {
    undoable: boolean,
    redoable: boolean,
}

const DEFAULT_ARGS:Args = {
    undoable: true, 
    redoable: false,
}

export const HeaderController = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;


    return `<module-header-controller slot="controller" ${argsToAttrs(props)}>
    </module-header-controller>`
}

HeaderController.args = DEFAULT_ARGS;
