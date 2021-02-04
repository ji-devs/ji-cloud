import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/admin/images/meta/sections/search-image-display";
import { Mode } from "@elements/entry/admin/images/meta/sections/search-image-display";

export default {
    title: "Entry/Admin/Images/Meta/Sections"
}

interface Args {
    thumbnail:string,
    imagename:string,
    mode: Mode,
    active:boolean,
}

const DEFAULT_ARGS:Args = {
    thumbnail: "3971-small.png",
    imagename: "Moses part the Nile",
    mode:"published",
    active:false,
}

export const SearchImageDisplay = (props?:Partial<Args>) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {thumbnail, imagename, mode,active} = props

    return `<search-image-display mode="${mode}" ${argsToAttrs(props)} thumbnail="${thumbnail}" imagename="${imagename}" ${active && "active"}></search-image-display>`;
}

SearchImageDisplay.args = DEFAULT_ARGS;
SearchImageDisplay.argTypes = {
  Mode  : {
        control: {
            type: 'inline-radio',
            options: ["published", "saved"]
        }
    }
}