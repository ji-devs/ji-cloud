import "@elements/entry/admin/images/meta/sections/categories";
import {Container} from "./container";
import {mapToString} from "@utils/array";
import {DropdownTree} from "~/components/core/inputs/composed/dropdown-tree"
import {ReportTree} from "~/components/core/reports/tree"

export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}


export const ImageMetaCategories = () => {
    return Container({
        section: "categories",
        content: `
            <image-meta-section-categories>
              <div slot="category-select">${DropdownTree({mock: "images"})}</div>
              <div slot="category-report">${ReportTree({mock: "images"})}</div>

                <button-expand slot="expand"></button-expand>
            </image-meta-section-categories>
        `
    })
}
