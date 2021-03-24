import {argsToAttrs} from "@utils/attributes";
import "@elements/widgets/module-page/iframe";
import "@elements/module/memory/play/container";

import {Sidebar} from "./sections/sidebar";
import {Header} from "./sections/header";
import {Main} from "./sections/main";
import {mapToString, arrayIndex} from "@utils/array";

export default {
    title: "Module / Memory / Play"
}

interface Args {
    nCards: number
}

const DEFAULT_ARGS:Args = {
    nCards: 6
}

export const Player = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nCards} = props;

    return `
    <play-container slot="main">
        ${Sidebar({nPairs: nCards/2})} 
        ${Header()} 
        ${Main({nCards})} 
    </play-container>`;
}

Player.args = DEFAULT_ARGS;
