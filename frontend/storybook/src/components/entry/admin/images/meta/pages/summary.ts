import "@elements/entry/admin/images/meta/sections/summary";
import {Container} from "./container";
import {mapToString} from "@utils/array";
import {ReportTree} from "~/components/core/reports/tree"

export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}

const STR_EDIT = "Edit";

//Mock
const STR_IMGCONTENT = "Moses parts the Nile";
const STR_DESCRIPTION = "An open book, Moses hold his stick and raise his hands up, and part the Nile. An open book, Moses hold his stick and raise his hands up, and part the Nile. An open book, Moses hold his stick and raise his hands up, and part the Nile.   "
const STR_CLIPART = "Clipart";
const STR_DRAWING = "Drawing";
const STR_COMIC = "Comic";
const STR_CHABAD = "Chabad";
const STR_ALL = "All ages";
const STR_NO = " No religion";
const STR_REFORM = "Reform/Conservative";
const STR_ORTHODOX = "Orthodox";
const STR_NEXT = "Next"
export const ImageMetaSummary = () => {
    return Container({
        section: "summary",
        content: `
            <image-meta-section-summary>
            <div slot="category-report">${ReportTree({mock: "images"})}</div>
            <button-rect kind="text" color="blue" weight="medium" slot="edit">${STR_EDIT}</button-rect>


              <div slot="imagename">
                <title-ji color="black">${STR_IMGCONTENT}</title-ji>
              </div>
              <div slot="description">
                <title-ji color="black">${STR_DESCRIPTION}</title-ji>
              </div>
              <div slot="style">
                  <title-ji color="black">${STR_CLIPART}</title-ji>
                  <title-ji color="black">${STR_DRAWING}</title-ji>
                  <title-ji color="black">${STR_COMIC}</title-ji>
              </div>
              <div slot="used">
                <title-ji color="black">${STR_CHABAD}</title-ji>
              </div>
              <div slot="age">
                <title-ji color="black">${STR_ALL}</title-ji>
              </div>
              <div slot="stream">
                  <title-ji color="black">${STR_NO}</title-ji>
                  <title-ji color="black">${STR_REFORM}</title-ji>
                  <title-ji color="black">${STR_ORTHODOX}</title-ji>
              </div>
            </image-meta-section-summary>
        `
    })
}
