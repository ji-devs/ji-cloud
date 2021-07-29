import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/timer-indicator";

export default {
    title: "Entry / Jig / Play"
}

interface Args {
    value: string,
}

const DEFAULT_ARGS:Args = {
    value: "02:52",
}


export const TimerIndicator = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <div style="background:pink;padding:30px;">
            <jig-play-timer-indicator ${argsToAttrs(props)}></jig-play-timer-indicator>
        </div>
    `;
}

TimerIndicator.args = DEFAULT_ARGS;
