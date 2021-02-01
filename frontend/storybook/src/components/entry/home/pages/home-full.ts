import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/home/pages/home-full";
import { TopSection } from "~/components/entry/home/sections/homepage-top";
import { RecommendsSection } from "~/components/entry/home/sections/recommends-section";
import { CreateSection } from "~/components/entry/home/sections/create-section";
import { JigglingSection } from "~/components/entry/home/sections/jiggling-section";
import { whatsnewSection } from "~/components/entry/home/sections/whatsnew-section";
import { AboutUsSection } from "~/components/entry/home/sections/aboutus-section";
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
        ${RecommendsSection()}
        ${CreateSection()}
        ${JigglingSection()}
        ${whatsnewSection()}
        ${AboutUsSection()}
        ${Footer()}
    
    </homepage-full>`;
}

HomePageFull.args = DEFAULT_ARGS;