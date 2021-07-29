import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/home/search-results/search-result-details";

export default {
    title: "Entry / Home / Search results"
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Details = (props?:Args) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <home-search-result-details slot="collapsibles">
            <div>This game is about… using … Lorem Ipsum is simply dummy text of the printing and typesetting industry</div>
        </home-search-result-details>
    `;
}

Details.args = DEFAULT_ARGS;
