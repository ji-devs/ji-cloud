import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/admin/images/search/image-cell";
import { Mode } from "@elements/entry/admin/images/search/image-cell";
import { Ji as MockJiImage } from "~/components/core/images/ji";

export default {
    title: "Entry/Admin/Images/Search ",
};

interface Args {
    name: string;
    mode: Mode;
    active: boolean;
}

const DEFAULT_ARGS: Args = {
    name: "A chair",
    mode: "published",
    active: false,
};

export const ImageCell = (props?: Partial<Args> & { slot?: string }) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
    <search-image-cell ${argsToAttrs(props)}>
      ${MockJiImage({ size: "thumb", slot: "image" })}
    </search-image-cell >`;
};

ImageCell.args = DEFAULT_ARGS;
ImageCell.argTypes = {
    mode: {
        control: {
            type: "inline-radio",
            options: ["published", "saved"],
        },
    },
};
