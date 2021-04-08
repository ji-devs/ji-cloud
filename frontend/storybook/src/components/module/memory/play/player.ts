import {argsToAttrs} from "@utils/attributes";
import "@elements/widgets/module-page/iframe";
import "@elements/module/memory/play/container";

import {Sidebar} from "./sections/sidebar";
import {Main} from "./sections/main";
import {Ending} from "./sections/ending";
import {mapToString, arrayIndex} from "@utils/array";

export default {
    title: "Module / Memory / Play"
}

interface Args {
    nCards: number,
    isEnding: boolean
}

const DEFAULT_ARGS:Args = {
    nCards: 6,
    isEnding: false 
}

export const Player = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nCards, isEnding} = props;

    return `
    <play-container slot="main">
        ${Sidebar({nPairs: nCards/2})}
        ${isEnding 
            ? Ending()
            : Main({nCards})
        }
    </play-container>`;
}

Player.args = DEFAULT_ARGS;
