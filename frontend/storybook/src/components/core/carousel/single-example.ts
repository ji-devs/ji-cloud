import "@elements/mock/carousel-single-example";

export default {
    title: "Core / Carousel"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const SingleExample = (props?:Args) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <carousel-single-example></carousel-single-example>
    `;
}

SingleExample.args = DEFAULT_ARGS;

