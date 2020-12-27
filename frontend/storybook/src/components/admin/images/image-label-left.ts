import "@elements/admin/images/image-settings";
import "@elements/admin/images/image-label-left";
import "@elements/images/basic";
import "@elements/inputs/checkbox";

import "@elements/buttons/replace-delete";
import "@elements/dividers/vertical-full";
import {InputUnderlined} from "~/components/input";


import {mockJiImage} from "~/mock/images";
import "@elements/images/ji";
export default {
  title: 'Admin/Images/Settings',
}

interface Props {
  label: string
}

const DEFAULT_PROPS:Props = {
  label: "label here"
}

export const LeftLabel = (props?:Props) => {
    const {label} = props || DEFAULT_PROPS;

    return `
    <imagelabel-left>
      <img src="http://localhost:4102/ui/mock/thumbnail/thumbnail-image.jpg" slot="image"/>
      <replace-delete slot="image-actions"></replace-delete>
      <vertical-full slot=divider></vertical-full>
      <input-checkbox label="Premium Image" slot="checkbox"></input-checkbox>
      <input-underlined slot="description" label="Image name"></input-underlined>
      <textarea-underlined slot="description" label="Image description"></textarea-underlined>
    </image-settings>

    
    `
}

LeftLabel.args = DEFAULT_PROPS;