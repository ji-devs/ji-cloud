import "@elements/entry/jig/gallery/gallery";
import "@elements/entry/jig/gallery/create";
import "@elements/entry/jig/gallery/template";
import "@elements/entry/jig/gallery/recent";
import "@elements/core/inputs/composed/search";
import "@elements/core/menu/menu-line";
import { arrayCount, mapToString } from "@utils/array";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Entry / Jig / Gallery"
}

interface Args {
    recentCount: number;
}

const DEFAULT_ARGS:Args = {
    recentCount: 12,
}

export const Gallery = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-gallery ${argsToAttrs(props)}>
            <jig-gallery-create slot="create-jig"></jig-gallery-create>
            <jig-gallery-template slot="jig-templates" ages="5-8" kind="vocabulary"></jig-gallery-template>
            <jig-gallery-template slot="jig-templates" ages="5-8" kind="parsha"></jig-gallery-template>
            <jig-gallery-template slot="jig-templates" ages="5-8" kind="vocabulary"></jig-gallery-template>
            <input-select slot="filters" value="Show all my JIGs">
                <li-check selected>Show all my JIGs</li-check>
                <li-check>Show published Jigs</li-check>
                <li-check>Show drafts</li-check>
            </input-select>
            <input-search slot="search-input"></input-search>
            ${mapToString(arrayCount(props.recentCount), recent)}
        </jig-gallery>
    `;
}

Gallery.args = DEFAULT_ARGS;


function recent() {
    return `
        <jig-gallery-recent
            slot="recent-items"
            draft
            label="Teach New Vocabulary"
            img="mock/resized/jig-gallery.jpg"
            ages="5-8"
            lastEdited="3 W ago"
        >
            <menu-line slot="menu-content" icon="delete">Delete</menu-line>
        </jig-gallery-recent>
    `;
}