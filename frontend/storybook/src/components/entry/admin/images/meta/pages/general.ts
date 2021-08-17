import "@elements/entry/admin/images/meta/sections/general";
import {AGE_OPTIONS, STYLE_OPTIONS, AFFILIATION_OPTIONS} from "~/mock/meta";
import {Container} from "./container";
import {mapToString} from "@utils/array";

export default {
  title: 'Entry/Admin/Images/Meta/Pages',
}


export const ImageMetaGeneral = () => {
    return Container({
        section: "general",
        content: `
            <image-meta-section-general>
            ${mapToString(AGE_OPTIONS, label => {
                return `<input-checkbox slot="age_ranges" label="${label}"></input-checkbox>`
            })}
            ${mapToString(STYLE_OPTIONS, label => {
                return `<input-checkbox slot="styles" label="${label}"></input-checkbox>`
            })}
            ${mapToString(AFFILIATION_OPTIONS, label => {
                return `<input-checkbox slot="affiliations" label="${label}"></input-checkbox>`
            })}
            </image-meta-section-general>
        `
    })
}


