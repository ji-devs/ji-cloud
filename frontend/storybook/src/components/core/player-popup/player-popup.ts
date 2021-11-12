import { argsToAttrs } from "@utils/attributes";
import "@elements/core/player-popup/player-popup";

export default {
    title: "Core / Player popup",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const PlayerPopup = (props?: Args) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <player-popup ${argsToAttrs(props)}>
            <div slot="iframe" style="background: green;"></div>
            <button slot="close">×</button>
        </player-popup>
    `;
};

PlayerPopup.args = DEFAULT_ARGS;
