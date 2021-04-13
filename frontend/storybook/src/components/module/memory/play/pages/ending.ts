import {argsToAttrs} from "@utils/attributes";
import "@elements/widgets/module-page/iframe";
import "@elements/module/memory/play/container";
import {Player} from "../player";
import {mapToString, arrayIndex} from "@utils/array";

export default {
    title: "Module / Memory / Play / Pages"
}

interface Args {
    nCards: number,
}

const DEFAULT_ARGS:Args = {
    nCards: 28
}


export const Ending = (props?:Partial<Args>) => {
    return `<module-page-iframe>
    ${Player({...props, isEnding: true})}
    </module-page-iframe>
    `
}

Ending.args = DEFAULT_ARGS
