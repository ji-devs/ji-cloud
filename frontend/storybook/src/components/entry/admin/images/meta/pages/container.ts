import "@elements/entry/admin/images/meta/container";
import "@elements/entry/admin/images/meta/header";
import { Rectangle } from "~/components/core/buttons/rectangle";

import { Ji as MockJiImage } from "~/components/core/images/ji";
export default {
    title: "Entry/Admin/Images/Meta/Pages",
};

const STR_REPLACE = "Replace";
const STR_DELETE = "Delete";
const STR_PREMIUM = "Premium";
const STR_IMAGENAME = "Image name";
const STR_DESCRIPTION = "Image description";

const STR_NEXT = "Next";

export const Container = ({ content, section }) => {
    return `<image-meta-container>
      <image-meta-header section="${section}" slot="header"></image-meta-header>
      ${MockJiImage({ size: "thumb", slot: "image" })}
      <button-rect kind="text" slot="replace" color="blue" size="small">${STR_REPLACE}</button-rect>
      <button-rect kind="text" slot="delete" color="blue" size="small">${STR_DELETE}</button-rect>
      <input-checkbox label="${STR_PREMIUM}" slot="premium"></input-checkbox>
      <input-text-underline slot="description" label="${STR_IMAGENAME}"></input-text-underline>
      <input-textarea-underline slot="description" label="${STR_DESCRIPTION}"></input-textarea-underline>

        <div slot="next">
          ${Rectangle({
              color: "red",
              size: "medium",
              contents: STR_NEXT,
              bold: false,
              italic: false,
          })}
        </div>
      <div slot="right">
        ${content ? content : ""}
      </div>
    </image-meta-container>`;
};
