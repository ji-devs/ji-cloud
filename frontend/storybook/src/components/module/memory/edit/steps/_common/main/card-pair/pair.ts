import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/steps/_common/main/card-pair/pair";
import "@elements/core/buttons/icon";
import {Card} from "./card";

export default {
    title: "Module / Memory / Edit / Steps / Main / Card-Pair"
}

interface Args {
    hover: boolean,
    index: number
}

const DEFAULT_ARGS:Args = {
    hover: true,
    index: 3 
}

export const Pair = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<main-card-pair ${argsToAttrs(props)} >
    ${Card({slot: "left"})} ${Card({slot: "right"})}
    <button-icon icon="circle-x-blue" slot="close"></button-icon> 
    </main-card-pair>`
}

Pair.args = DEFAULT_ARGS;
