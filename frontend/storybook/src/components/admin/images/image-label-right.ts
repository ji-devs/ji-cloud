import {ImageStyle} from "~/components/lists/image-style";
import {Age} from "~/components/lists/age";
import {Stream} from "~/components/lists/stream";


import "@elements/lists/two-column-list";
import {InputUnderlined} from "~/components/input";

import "@elements/images/ji";
export default {
  title: 'Admin/Images/Settings',
}

export const LabelRight = () => {
    
    return `
    <twocolumn-list>
     <div slot="left">${ImageStyle()}</div>
     <div slot="right">
      <input-underlined label="To be used only for"></input-underlined>
      ${
        Age() +
        Stream()
      
      }
     </div>

    </twocolumn-list>

    
    `
}
