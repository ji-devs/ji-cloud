import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/admin/images/search/pages/image-search";
import "@elements/core/menu/search-pagination";
import "@elements/core/menu/ellipses-menu";
import "@elements/core/inputs/dropdowns/dropdown-underlined";
import "@elements/entry/admin/images/meta/widgets/image-menu";

import { SearchImageDisplay } from "~/components/entry/admin/images/search/widgets/search-image-display";
export default {
    title: "Entry/Admin/Images/Search/Pages"
}

interface Args {
    results:string,
    number:string,
    searchword:string,
    active:boolean,
    closed:boolean
    
}

const DEFAULT_ARGS:Args = {
    results: "35",
    number: "5",
    searchword:"Pesach",
    active:false,
    closed:false,
   
}

export const ImageSearch = (props?:Partial<Args>) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {results,number, searchword} = props

    return `<image-search ${argsToAttrs(props)}>
    <title-ji color="black" size="medium-large" slot="number">${number}&nbsp;</title-ji>
    <title-ji color="black" size="medium-large" slot="searchword">${searchword}&nbsp;</title-ji>

    <search-pagination slot="pagination">
    <span>${results}</span>
    
    </search-pagination>
    <dropdown-underlined slot="dropdown">
        <image-menu></image-menu>
    </dropdown-underlined>
    <div slot="image-display">
    ${SearchImageDisplay({active:true})}
    </div>
    <div slot="image-display">
    ${SearchImageDisplay({mode:"saved"})}
    </div>
    <div slot="image-display">
    ${SearchImageDisplay()}
    </div>
    <div slot="image-display">
    ${SearchImageDisplay()}
    </div>
    <div slot="image-display">
    ${SearchImageDisplay()}
    </div>
    <div slot="image-display">
    ${SearchImageDisplay()}
    </div>
    <search-pagination slot="pagination-bottom">
    </image-search>`;
}

ImageSearch.args = DEFAULT_ARGS;