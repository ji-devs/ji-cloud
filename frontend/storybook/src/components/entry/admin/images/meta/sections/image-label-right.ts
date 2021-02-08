import {ImageStyle} from "~/components/entry/admin/images/meta/lists/image-style";
import {Age} from "~/components/entry/admin/images/meta/lists/age";
import {Stream} from "~/components/entry/admin/images/meta/lists/stream";
import "@elements/core/inputs/text-underline";
import "@elements/core/lists/list-two-column";
import "@elements/core/images/ji";

export default {
  title: 'Entry/Admin/Images/Meta/Sections',
}


export const LabelRight = () => {
    
    return `
    <list-two-column>
     <div slot="one">${ImageStyle()}</div>
     <div slot="two">
      ${
        Age() +
        Stream()
      
      }
     </div>

    </list-two-column>

    
    `
}
