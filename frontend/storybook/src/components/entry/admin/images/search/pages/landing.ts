import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/admin/images/search/pages/landing";
import { mapToString, arrayCount } from "@utils/array";
import { ImageCell } from "../image-cell";
export default {
    title: "Entry/Admin/Images/Search/Pages",
};

const STR_SHOW_ALL = "Show all";

interface Args {}

const DEFAULT_ARGS: Args = {
    query: "Pesach",
    nResults: 10,
};

export const Landing = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
    <image-search ${argsToAttrs(props)}>
    <pagination-widget page="1" total="4" slot="pagination-top"></pagination-widget>
        <dropdown-underlined value="${STR_SHOW_ALL}" slot="publish-filter" open>
            <image-search-publish-filter slot="options" mode="all"></image-search-publish-filter>
            <image-search-publish-filter slot="options" mode="published"></image-search-publish-filter>
            <image-search-publish-filter slot="options" mode="saved"></image-search-publish-filter>
        </dropdown-underlined>
        <pagination-widget page="1" total="4" slot="pagination-bottom"></pagination-widget>
        ${mapToString(arrayCount((props as any).nResults), (idx) => {
            return ImageCell({ slot: "images" });
        })}
    </image-search>`;
};

Landing.args = DEFAULT_ARGS;
