import "@elements/entry/home/home/whats-new/home-whats-new";
import "@elements/core/titles/variants/title-section";
import { WhatsNewItem } from "./whats-new-item";
import { arrayCount, mapToString } from "@utils/array";

export default {
    title: "Entry / Home / Home / What's new",
};

interface Args {
    nResults: number;
}

const DEFAULT_ARGS: Args = {
    nResults: 3,
};

export const WhatsNew = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <home-whats-new pageCount="${props.nResults}">
            ${mapToString(arrayCount((props as any).nResults), (idx) => {
                return WhatsNewItem();
            })}
        </home-whats-new>
    `;
};
WhatsNew.args = DEFAULT_ARGS;
