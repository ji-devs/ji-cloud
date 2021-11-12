import "@elements/entry/home/home/testimonials/testimonial-carousel";
import { arrayCount, mapToString } from "@utils/array";
import { TestimonialItem } from "./testimonial-item";

export default {
    title: "Entry / Home / Home / Testimonial",
};

interface Args {
    slot?: string;
    nResults: number;
}

const DEFAULT_ARGS: Args = {
    nResults: 4,
};

export const TestimonialCarousel = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
    ${props.nResults}
        <home-testimonial-carousel ${props.slot || ""} pageCount="${
        props.nResults
    }">
            ${mapToString(arrayCount((props as any).nResults), () => {
                return TestimonialItem();
            })}
        </home-testimonial-carousel>
    `;
};
TestimonialCarousel.args = DEFAULT_ARGS;
