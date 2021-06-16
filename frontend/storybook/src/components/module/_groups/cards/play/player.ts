import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_groups/cards/play/container";

import {Sidebar} from "./sidebar";
import {Main} from "./main";
import {Ending} from "./ending";
import {mapToString, arrayIndex} from "@utils/array";

export default {
    title: "Module / _GROUPS / Cards / play"
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
