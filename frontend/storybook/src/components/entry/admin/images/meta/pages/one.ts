import "@elements/entry/admin/images/meta/page";
import { LeftLabel } from "~/components/entry/admin/images/meta/image-label-left";
import {LabelRight} from "~/components/entry/admin/images/meta/image-label-right";
import "@elements/core/titles/variants/underlined-title";
import { Rectangle } from "~/components/core/buttons/rectangle";

export default {
  title: 'Admin/Image Label',
}

interface Args {
}

const DEFAULT_ARGS:Args = {
}


const STR_NEXT = "Next";
const STR_LABEL ="Label Images"

export const ImageLabelFullOne = (props?:Partial<Args>) => {

  props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
  const {} = props

  return `<image-meta-page>
    <underlined-title slot="title" title=${STR_LABEL}></underlined-title>
    <div slot="left">${LeftLabel()}</div>
    <div slot="middle">${LabelRight()}</div>
    <div slot="button">
      ${Rectangle({color:"red",size:"medium",contents:STR_NEXT,bold:false, italic:false})}
    </div>

  </image-meta-page>`;
}

ImageLabelFullOne.args = DEFAULT_ARGS;

