import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/replay";

export default {
    title: "Entry / Jig / Play"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}


export const Replay = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-play-replay ${argsToAttrs(props)}></jig-play-replay>
    `;
}

Replay.args = DEFAULT_ARGS;