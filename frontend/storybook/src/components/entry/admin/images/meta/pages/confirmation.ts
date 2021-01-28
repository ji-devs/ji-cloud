import "@elements/entry/admin/images/meta/page";
import "@elements/core/titles/variants/underlined-title";
import "@elements/entry/admin/images/add/add-image";
import { Rectangle } from "~/components/core/buttons/rectangle";

export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}

const STR_LABEL = "Label Images";
const STR_UPLOAD = "Upload image";


export const Confirmation = () => {
    return `
    <image-meta-page>
      <underlined-title slot="title" title=${STR_LABEL}></underlined-title>
      <div slot="left">
        <add-image>
            ${Rectangle({color:"blue",size:"medium",contents:STR_UPLOAD,bold:false, italic:false, iconBefore:"create"})}
        </add-image>
      </div>
      
    </image-meta-page>
    
    `
}

Confirmation.args = {
 title: "Label Images",
 iconBefore:"create"
}
