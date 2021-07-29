import "@elements/entry/admin/images/meta/pages/landing";
import "@elements/entry/admin/images/meta/sections/section-2";
import {Landing} from "./landing";
import {mapToString} from "@utils/array";
import {DropdownTree} from "~/components/core/inputs/composed/dropdown-tree"
import {ReportTree} from "~/components/core/reports/tree"

export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}


export const ImageMeta2 = () => {
    return Landing({
        content: `
            <image-meta-section-2>
              <div slot="category-select">${DropdownTree({mock: "images"})}</div>
              <div slot="category-report">${ReportTree({mock: "images"})}</div>

                <button-expand slot="expand"></button-expand>
            </image-meta-section-2>
        `
    })
}
