import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/play/sections/main";
import {Card} from "../card"; 
import {mapToString, arrayIndex} from "@utils/array";

const N_CARD_OPTIONS = [8,10,12,14,16,18,20,22,24,26,28];

export default {
    title: "Module / Memory / Play / Sections"
}

interface Args {
    nCards: number,
}

const DEFAULT_ARGS:Args = {
    nCards: 28
}

export const Main = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {nCards} = props;

    return `
    <play-main ${argsToAttrs(props)} slot="main">
        ${mapToString(arrayIndex(nCards), index => {
            return Card({
                flipped: index % 2 == 0,
                contentMode: index % 4 == 0 ? "text" : "image"
            })
        })}
    </play-main>`;
}

Main.args = DEFAULT_ARGS;

Main.argTypes = {
    nCards: {
        control: {
            type: 'inline-radio',
            options: N_CARD_OPTIONS
        }
    }
}
