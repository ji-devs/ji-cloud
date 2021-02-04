import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/admin/images/meta/image-search";
import "@elements/core/menu/search-pagination";
import { SearchImageDisplay } from "~/components/entry/admin/images/meta/sections/search-image-display";
export default {
    title: "Entry/Admin/Images/Meta/Pages"
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

    return `<image-search ${argsToAttrs(props)} ${closed && "closed"}>
    <title-ji color="black" size="medium-large" slot="number">${number}&nbsp;</title-ji>
    <title-ji color="black" size="medium-large" slot="searchword">${searchword}&nbsp;</title-ji>

    <search-pagination slot="pagination" ${closed && "closed"}>
    <span>${results}</span>
    </search-pagination>
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