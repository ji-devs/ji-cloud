import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/main-empty";
import "@elements/module/memory/edit/steps/sections/main/main";
import "@elements/widgets/module-page/iframe";
import {mapToString, arrayIndex} from "@utils/array";
import {Pair} from "~/components/module/memory/edit/steps/sections/main/card-pair/pair";
import {Args as CardArgs} from "~/components/module/memory/edit/steps/sections/main/card-pair/card";
export default {
    title: "Module / Memory / Play"
}

interface Args {
    nCards: number
}

const DEFAULT_ARGS:Args = {
    nCards: 6
}

export const Page = (props?:Partial<Args>) => {
    return `<module-page-iframe>
    ${Player()}
    </module-page-iframe>
    `
}
export const Player = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<div slot="main">Player here!</div>`;
}

Player.args = DEFAULT_ARGS;
