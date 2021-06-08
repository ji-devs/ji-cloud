import "@elements/entry/home/testimonials/testimonial-item";


export default {
    title: 'Entry/ Home / Testimonial',
}

interface Args {
    header: string,
    paragraph: string,
    slot?: string,
}

const DEFAULT_ARGS: Args = {
    header: "Sarah Nazirah, Mexico",
    paragraph: "I want to tell you, because of JI, my children are learning Hebrew and English simultaneously. For my children, you are not only teaching two children, yo are also saving their souls. I reaffirm that they have achieved educational rehabilitation, thanks to JI."
}

export const TestimonialItem = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <home-testimonial-item ${props.slot ? `slot="${props.slot}"` : ""}>
            <img-ji slot="image" id="face-round.webp" lib="mock" size="original"></img-ji>
            <h4 slot="header">${props.header}</h4>
            <p slot="paragraph">${props.paragraph}</p>
        </home-testimonial-item>
    `
}
TestimonialItem.args = DEFAULT_ARGS;
