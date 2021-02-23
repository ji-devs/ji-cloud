import "@elements/entry/admin/images/meta/pages/landing";
import "@elements/entry/admin/images/meta/sections/section-1";
import {AGE_OPTIONS, STYLE_OPTIONS, AFFILIATION_OPTIONS} from "~/mock/meta";
import {Landing} from "./landing";
import {mapToString} from "@utils/array";

export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}


export const ImageMeta1 = () => {
    return Landing({
        content: `
            <image-meta-section-1>
            ${mapToString(AGE_OPTIONS, label => {
                return `<input-checkbox slot="ages" label="${label}"></input-checkbox>`
            })}
            ${mapToString(STYLE_OPTIONS, label => {
                return `<input-checkbox slot="styles" label="${label}"></input-checkbox>`
            })}
            ${mapToString(AFFILIATION_OPTIONS, label => {
                return `<input-checkbox slot="affiliations" label="${label}"></input-checkbox>`
            })}
            </image-meta-section-1>
        `
    })
}


