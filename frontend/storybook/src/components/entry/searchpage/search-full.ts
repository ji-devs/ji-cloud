import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/search/search-full";
import "@elements/entry/search/search-header";
import "@elements/entry/home/sections/footer-section";
import { Type } from "@elements/entry/search/card-front";

import { SearchHeader } from "~/components/entry/searchpage/search-header";
import { SearchCardFront } from "~/components/entry/searchpage/card-front";
import { SearchCardBack } from "~/components/entry/searchpage/card-back";


export default {
    title: "Entry/Home/Search"
}

interface Args {
    jignumber: string,
    learningnumber: string,
    recommendednumber: string,
}

const DEFAULT_ARGS: Args = {
    jignumber: "(283)",
    learningnumber: "(58)",
    recommendednumber: "(4)"
}

const STR_RESULTS = "341";
const STR_FOUND = "Hebrew";
export const SearchFull = (props?: Partial<Args>) => {

    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;
    const { } = props

    return `<search-full ${argsToAttrs(props)}>
    <div slot="header">
        ${SearchHeader()}
    </div>
    <title-ji slot="results" color="red" size="title-large" weight="bold">${STR_RESULTS}</title-ji>
    <title-ji slot="phrase" color="red" size="title-large" weight="bold">${STR_FOUND}</title-ji>

    <div slot="card">
     ${SearchCardFront({type:"new"})}
    </div>
    <div slot="card">
    ${SearchCardFront({type:"label"})}
   </div>
   <div slot="card">
   ${SearchCardFront()}
  </div>
  <div slot="card">
  ${SearchCardBack()}
    </div>
    <div slot="card">
    ${SearchCardFront()}
    </div>
    <div slot="card">
    ${SearchCardFront()}
    </div>
    <div slot="card">
    ${SearchCardFront()}
    </div>
    <div slot="card">
    ${SearchCardFront()}
    </div>
    <div slot="learncard">
    ${SearchCardFront()}
    </div>
    <div slot="learncard">
    ${SearchCardFront()}
    </div>
    <div slot="learncard">
    ${SearchCardFront()}
    </div>
    <div slot="learncard">
    ${SearchCardFront()}
    </div>
    <div slot="recommendedcard">
    ${SearchCardFront()}
    </div>
    <div slot="recommendedcard">
    ${SearchCardFront()}
    </div>
    <div slot="recommendedcard">
    ${SearchCardFront()}
    </div>
    <div slot="recommendedcard">
    ${SearchCardFront()}
    </div>
    <footer-section slot="footer"></footer-section>
    <search-full/>`;
}

SearchFull.args = DEFAULT_ARGS;