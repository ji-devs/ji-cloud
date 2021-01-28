import "@elements/entry/admin/images/meta/sections/left";
import "@elements/core/inputs/checkbox";
import "@elements/core/inputs/text-underline";
import "@elements/core/inputs/textarea-underline";
import "@elements/core/buttons/text";
import "@elements/core/dividers/horizontal-full";

import {Ji as MockJiImage} from "~/components/core/images/ji";
import "@elements/core/images/ji";
export default {
  title: 'Entry/Admin/Images/Meta/Sections',
}

interface Props {
  
}

const DEFAULT_PROPS:Props = {
}

const STR_REPLACE ="Replace";
const STR_DELETE = "Delete";
const STR_PREMIUM ="Premium Image";
const STR_IMAGENAME = "Image name";
const STR_DESCRIPTION = "Image description"

export const LeftLabel = (props?:Partial<Props>) => {

    return `
    <section-left>
      ${MockJiImage({size: "thumb", slot: "image"})}
      <button-text slot="image-actions" color="blue" size="small">${STR_REPLACE}</button-text>
      <horizontal-full slot="image-actions" color="blue"></horizontal-full>
      <button-text slot="image-actions" color="blue" size="small">${STR_DELETE}</button-text>
      <input-checkbox label="${STR_PREMIUM}" slot="checkbox"></input-checkbox>
      <input-text-underline slot="description" label="${STR_IMAGENAME}"></input-text-underline>
      <input-textarea-underline slot="description" label="${STR_DESCRIPTION}"></input-textarea-underline>
    </section-left>

    
    `
}

LeftLabel.args = DEFAULT_PROPS;