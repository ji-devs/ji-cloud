import { argsToAttrs } from "@utils/attributes";
import "@elements/module/video/third-party-input-card";
import { Host } from "@elements/module/video/third-party-input-card";

export default {
    title: "Module / Video / Edit",
};

interface Args {
    host: Host;
}

const DEFAULT_ARGS: Args = {
    host: "youtube",
};

export const ThirdPartyInputCard = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <video-third-party-input-card ${argsToAttrs(props)}>
            <input-wrapper slot="input" label="Add a YouTube link">
                <input-youtube></input-youtube>
            </input-wrapper>
            <button-rect slot="delete" kind="text" color="blue">Delete</button-rect>
        </video-third-party-input-card>
    `;
};

ThirdPartyInputCard.args = DEFAULT_ARGS;
ThirdPartyInputCard.argTypes = {
    nPairs: {
        control: {
            type: "inline-radio",
            options: ["youtube"],
        },
    },
};
