import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/admin/images/search/widgets/search-image-display";
import { Mode } from "@elements/entry/admin/images/search/widgets/search-image-display";
import {Ji as MockJiImage} from "~/components/core/images/ji";

export default {
    title: "Entry/Admin/Images/Search/Widgets"
}

interface Args {
    name:string,
    mode: Mode,
    active:boolean,
}

const DEFAULT_ARGS:Args = {
    name: "A chair",
    mode:"published",
    active:false,
}

export const SearchImageDisplay = (props?:Partial<Args>) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
    <search-image-display ${argsToAttrs(props)}>
      ${MockJiImage({size: "thumb", slot: "image"})}
    </search-image-display>`;
}

SearchImageDisplay.args = DEFAULT_ARGS;
SearchImageDisplay.argTypes = {
  mode  : {
        control: {
            type: 'inline-radio',
            options: ["published", "saved"]
        }
    }
}
