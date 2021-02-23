import "@elements/entry/admin/images/meta/pages/landing";
import { Rectangle } from "~/components/core/buttons/rectangle";

import {Ji as MockJiImage} from "~/components/core/images/ji";
export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}


const STR_REPLACE ="Replace";
const STR_DELETE = "Delete";
const STR_PREMIUM ="Premium";
const STR_IMAGENAME = "Image name";
const STR_DESCRIPTION = "Image description"

const STR_NEXT = "Next";


export const Landing = ({content}) => {
    return `<image-meta-page>
      ${MockJiImage({size: "thumb", slot: "image"})}
      <button-text slot="replace" color="blue" size="small">${STR_REPLACE}</button-text>
      <button-text slot="delete" color="blue" size="small">${STR_DELETE}</button-text>
      <input-checkbox label="${STR_PREMIUM}" slot="premium"></input-checkbox>
      <input-text-underline slot="description" label="${STR_IMAGENAME}"></input-text-underline>
      <input-textarea-underline slot="description" label="${STR_DESCRIPTION}"></input-textarea-underline>

        <div slot="next">
          ${Rectangle({color:"red",size:"medium",contents:STR_NEXT,bold:false, italic:false})}
        </div>
      <div slot="right">
        ${content ? content : ""}
      </div>
    </image-meta-page>`;
}


