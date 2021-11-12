import "@elements/core/images/ji";
import { argsToAttrs } from "@utils/attributes";
import { MediaLibOptions, MediaSizeOptions } from "@utils/path";
import { injectSlotStr } from "@utils/slot";

export default {
    title: "Core / Images",
};

/*** Ji - mock ****/

interface Args {
    lib: MediaLibOptions;
    size: MediaSizeOptions;
    id: string;
    fallback: boolean;
    slot?: string;
}

const DEFAULT_ARGS: Args = {
    lib: "mock",
    id: "tall.png",
    size: "full",
    fallback: false,
};

export const Ji = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    const { slotStr, fallback, ...imageProps } = injectSlotStr(props);
    return `<img-ji ${argsToAttrs(imageProps)} ${slotStr}>
    ${fallback ? renderFallback() : ""} 
  </img-ji>`;
};

function renderFallback() {
    return `<img-ui path="core/cards/icon-group.svg" slot="fallback"></img-ui>`;
}

Ji.argTypes = {
    lib: {
        control: {
            type: "inline-radio",
            options: ["global", "user", "web", "screenshot", "mock"],
        },
    },
    size: {
        control: {
            type: "inline-radio",
            options: ["original", "full", "thumb"],
        },
    },
};

Ji.args = DEFAULT_ARGS;
