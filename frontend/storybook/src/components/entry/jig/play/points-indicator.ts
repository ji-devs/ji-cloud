import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/play/points-indicator";

export default {
    title: "Entry / Jig / Play"
}

interface Args {
    value: string,
}

const DEFAULT_ARGS:Args = {
    value: "210",
}


export const PointsIndicator = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <div style="background:pink;padding:30px;">
            <jig-play-points-indicator ${argsToAttrs(props)}></jig-play-points-indicator>
        </div>
    `;
}
PointsIndicator.args = DEFAULT_ARGS;
