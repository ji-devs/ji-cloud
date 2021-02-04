import "@elements/entry/admin/images/meta/page";
import "@elements/core/titles/variants/underlined-title";
import "@elements/entry/admin/images/add/add-image";

export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}


const STR_LABEL = "Label Images";




export const AddImage = () => {
    return `
    <image-meta-page>
      <underlined-title slot="title" title=${STR_LABEL}></underlined-title>
      <div slot="left">
        <add-image>
        </add-image>
      </div>
      
    </image-meta-page>
    
    `
}

