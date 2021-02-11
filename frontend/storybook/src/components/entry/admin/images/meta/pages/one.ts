import "@elements/entry/admin/images/meta/pages/page";
import { LeftLabel } from "~/components/entry/admin/images/meta/sections/image-label-left";
import {LabelRight} from "~/components/entry/admin/images/meta/sections/image-label-right";

import { Rectangle } from "~/components/core/buttons/rectangle";

export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}


const STR_NEXT = "Next";


export const ImageMeta1 = () => {




  return `<image-meta-page>
    <div slot="left">${LeftLabel()}</div>
    <div slot="middle">${LabelRight()}</div>
    <div slot="button">
      ${Rectangle({color:"red",size:"medium",contents:STR_NEXT,bold:false, italic:false})}
    </div>

  </image-meta-page>`;
}


