import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/home/home/home-full";
import { SearchSection } from "~/components/entry/home/home/search-section/search-section";
import { QuickSearch } from "~/components/entry/home/home/quick-search/quick-search";
import { Create } from "~/components/entry/home/home/create";
import { WhyJi } from "~/components/entry/home/home/why-ji/why-ji";
import { WhatsNew } from "~/components/entry/home/home/whats-new/whats-new";
import { Testimonials } from "~/components/entry/home/home/testimonials/testimonials";
import { PageFooter } from "~/components/core/page-footer/page-footer";
import { PageHeader } from "~/components/core/page-header/page-header";
import { SearchResults } from "~/components/entry/home/home/search-results/search-results";

export default {
    title: "Entry / Home / Home"
}

interface Args {
    page: string,
}

const DEFAULT_ARGS:Args = {
    page: "home",
}

export const HomePageFull = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {} = props

    return `<home-full ${argsToAttrs(props)}>
        ${PageHeader()}
        ${SearchSection({mode: props.page as any})}
        ${ 
            props.page === "home" ? (
                QuickSearch() +
                Create() +
                WhyJi() +
                WhatsNew() +
                Testimonials()
            ) : (
                SearchResults()
            )
        }
        ${PageFooter()}
    
    </home-full>`;
}

HomePageFull.args = DEFAULT_ARGS;
HomePageFull.argTypes = {
    page: {
        control: {
            type: 'inline-radio',
            options: ["home", "results"]
        }
    }
}