import {argsToAttrs} from "@utils/attributes";
import "@elements/module/memory/edit/choose/pages/landing";

export default {
    title: "Module / Memory / Edit / Choose / Pages"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Landing = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    
    return `
    <choose-page>
        <choose-card mode="duplicate"></choose-card>
        <choose-card mode="words-images"></choose-card>
        <choose-card mode="begins"></choose-card>
        <choose-card mode="lettering"></choose-card>
        </choose-page>
    `;
}

//options: ["duplicate", "words-images", "begins", "lettering"]
Landing.args = DEFAULT_ARGS;
