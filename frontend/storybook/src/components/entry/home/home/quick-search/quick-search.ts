import "@elements/entry/home/home/quick-search/quick-search";
import { QuickSearchItem } from "./quick-search-item";

export default {
    title: 'Entry / Home / Home / Quick search',
}

export const QuickSearch = () => {
    return `
        <home-quick-search>
            ${QuickSearchItem()}
            ${QuickSearchItem()}
            ${QuickSearchItem()}
            ${QuickSearchItem()}
            ${QuickSearchItem()}
        </home-quick-search>
    `
}
