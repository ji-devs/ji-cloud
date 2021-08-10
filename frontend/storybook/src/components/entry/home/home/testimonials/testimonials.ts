import "@elements/entry/home/home/testimonials/testimonials";
import { TestimonialItem } from "./testimonial-item";
import { arrayCount, mapToString } from "@utils/array";



export default {
    title: 'Entry / Home / Home / Testimonial',
}

interface Args {
}

const DEFAULT_ARGS: Args = {
    color: "yellow",
    size: "medium",
}

export const Testimonials = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <home-testimonials teachersPageCount="5" parentsPageCount="5">
            ${mapToString(arrayCount(5), () => {
                return TestimonialItem({slot: "parents"})
            })}
            ${mapToString(arrayCount(5), () => {
                return TestimonialItem({slot: "teachers"})
            })}
        </home-testimonials>
    `
}
Testimonials.args = DEFAULT_ARGS;
