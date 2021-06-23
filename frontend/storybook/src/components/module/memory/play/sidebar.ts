import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/play/sidebar";
import {mapToString, arrayIndex} from "@utils/array";
import {Card} from "../../_groups/cards/play/card";

const N_PAIR_OPTIONS = 
    [8,10,12,14,16,18,20,22,24,26,28]
    .map(nCards => nCards/2);

export default {
    title: "Module / Memory / Play" 
}

interface Args {
    nPairs: number
}

const DEFAULT_ARGS:Args = {
    nPairs: N_PAIR_OPTIONS[N_PAIR_OPTIONS.length-1]
}

export const Sidebar = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nPairs} = props;

    const pairs = makePairs(nPairs);

    return `
    <play-sidebar slot="sidebar">
        ${mapToString(pairs, ([card1, card2]) => {
            return Card(card1) + Card(card2);
        })}
    </play-sidebar>`;
}

function makePairs(nPairs:number):Array<any> {

    return arrayIndex(nPairs)
    .map(index => {
        const x = (index % 2) * 280;
        const y = 100 + ((Math.floor(index / 2)) * 280);
            const pair = [
                {scale: .5, translateX: x, translateY: y, transform: true},
                {scale: .5, translateX: x + 10, translateY: y + 10, transform: true},
            ];

            return pair;
        });
}

Sidebar.args = DEFAULT_ARGS;
Sidebar.argTypes = {
    nPairs: {
        control: {
            type: 'inline-radio',
            options: N_PAIR_OPTIONS
        }
    }
}
