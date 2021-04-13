import {argsToAttrs} from "@utils/attributes";
import "@elements/widgets/module-page/iframe";
import "@elements/module/memory/play/container";
import {Player} from "../player";
import {mapToString, arrayIndex} from "@utils/array";

const N_CARD_OPTIONS = [8,10,12,14,16,18,20,22,24,26,28];
export default {
    title: "Module / Memory / Play / Pages"
}

interface Args {
    nCards: number,
}

const DEFAULT_ARGS:Args = {
    nCards: 28
}


export const Landing = (props?:Partial<Args>) => {
    return `<module-page-iframe>
    ${Player({...props, isEnding: false})}
    </module-page-iframe>
    `
}

Landing.args = DEFAULT_ARGS
Landing.argTypes = {
    nCards: {
        control: {
            type: 'inline-radio',
            options: N_CARD_OPTIONS
        }
    }
}
