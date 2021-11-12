import "@elements/core/carousel/multi";
import { arrayCount, mapToString } from "@utils/array";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Core / Carousel",
};

interface Args {
    nItems: number;
    itemWidth: number;
    itemHeight: number;
}

const DEFAULT_ARGS: Args = {
    nItems: 15,
    itemWidth: 300,
    itemHeight: 300,
};

export const Multi = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    const { nItems, itemWidth, itemHeight, ...carouselProps } = props;

    return `<carousel-multi ${argsToAttrs(carouselProps)}>
        ${mapToString(
            arrayCount(nItems),
            (i) =>
                `<div style="width: ${itemWidth}px; height: ${itemHeight}px; background-color: #1ea7fd; text-align: center;">Item ${i}</div>`
        )}
    </carousel-multi>`;
};

Multi.args = DEFAULT_ARGS;
