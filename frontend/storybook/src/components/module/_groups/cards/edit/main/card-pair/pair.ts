import { argsToAttrs } from "@utils/attributes";
import "@elements/core/buttons/icon";
import { Card, Args as CardArgs } from "./card";

import "@elements/module/_groups/cards/edit/main/card-pair/pair";

export default {
    title: "Module / _GROUPS / Cards / edit / Main",
};

interface Args {
    hover: boolean;
    index: number;
    leftArgs?: CardArgs;
    rightArgs?: CardArgs;
}

const DEFAULT_ARGS: Args = {
    hover: true,
    index: 3,
};

export const Pair = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    const { leftArgs, rightArgs, ...pairProps } = props;

    return `<main-card-pair ${argsToAttrs(pairProps)} >
    ${Card({ ...leftArgs, slot: "left" })} ${Card({
        ...rightArgs,
        slot: "right",
    })}

    <button-icon icon="circle-x-blue" slot="close"></button-icon> 
    </main-card-pair>`;
};

Pair.args = DEFAULT_ARGS;
