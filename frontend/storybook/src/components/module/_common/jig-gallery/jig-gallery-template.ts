
import "@elements/module/_common/jig-gallery/jig-gallery-template";
import { Kind } from "@elements/module/_common/jig-gallery/jig-gallery-template";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Jig Gallery"
}

interface Args {
    kind: Kind;
    ages: string;
}

const DEFAULT_ARGS:Args = {
    kind: "vocabulary",
    ages: "3-6",
}

export const JigGalleryTemplate = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-gallery-template ${argsToAttrs(props)}></jig-gallery-template>
    `;
}

JigGalleryTemplate.args = DEFAULT_ARGS;
JigGalleryTemplate.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ["vocabulary", "parsha"]
        }
    }
}
