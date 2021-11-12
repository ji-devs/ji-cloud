import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/jig/play/background-music";

export default {
    title: "Entry / Jig / Play",
};

interface Args {
    playing: boolean;
    disabled: boolean;
}

const DEFAULT_ARGS: Args = {
    playing: true,
    disabled: false,
};

export const BackgroundMusic = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <jig-play-background-music ${argsToAttrs(
            props
        )}></jig-play-background-music>
    `;
};
BackgroundMusic.args = DEFAULT_ARGS;
