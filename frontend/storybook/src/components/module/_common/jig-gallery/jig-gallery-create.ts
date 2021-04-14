import "@elements/module/_common/jig-gallery/jig-gallery-create";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Jig Gallery"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const JigGalleryCreate = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-gallery-create ${argsToAttrs(props)}></jig-gallery-create>
    `;
}

JigGalleryCreate.args = DEFAULT_ARGS;
