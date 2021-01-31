import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/home/pages/home-full";
import { TopSection } from "~/components/entry/home/sections/homepage-top";
import { RecommendsParagraph } from "~/components/entry/home/sections/recommends-paragraph";
import { CreateParagraph } from "~/components/entry/home/sections/create-paragraph";
import { JigglingParagraph } from "~/components/entry/home/sections/jiggling-paragraph";
import { whatsnewParagraph } from "~/components/entry/home/sections/whatsnew-paragraph";
import { AboutUsParagraph } from "~/components/entry/home/sections/aboutus-paragraph";
  import { Footer } from "~/components/entry/home/sections/footer";

export default {
    title: "Entry/ Home"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const HomePageFull = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<homepage-full ${argsToAttrs(props)}>
        ${TopSection()}
        ${RecommendsParagraph()}
        ${CreateParagraph()}
        ${JigglingParagraph()}
        ${whatsnewParagraph()}
        ${AboutUsParagraph()}
        ${Footer()}
    
    </homepage-full>`;
}

HomePageFull.args = DEFAULT_ARGS;