import "@elements/core/carousel/single";

export default {
    title: "Core / Carousel",
};

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Single = (props?: Args) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <carousel-single></carousel-single>
    `;
};

Single.args = DEFAULT_ARGS;
