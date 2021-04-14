import "@elements/module/_common/jig-gallery/jig-gallery";
import "@elements/module/_common/jig-gallery/jig-gallery-create";
import "@elements/module/_common/jig-gallery/jig-gallery-template";
import "@elements/module/_common/jig-gallery/jig-gallery-recent";
import "@elements/core/buttons/text";
import "@elements/core/inputs/search";
import "@elements/core/menu/menu-line";
import { arrayCount, mapToString } from "@utils/array";
import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Module / _common / Jig Gallery"
}

interface Args {
    recentCount: number;
}

const DEFAULT_ARGS:Args = {
    recentCount: 12,
}

export const JigGallery = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <jig-gallery ${argsToAttrs(props)}>
            <jig-gallery-create slot="craete-jig"></jig-gallery-create>
            <jig-gallery-template slot="jig-templates" ages="5-8" kind="vocabulary"></jig-gallery-template>
            <jig-gallery-template slot="jig-templates" ages="5-8" kind="parsha"></jig-gallery-template>
            <jig-gallery-template slot="jig-templates" ages="5-8" kind="vocabulary"></jig-gallery-template>
            <button-text slot="see-all-templates-button">See all templates</button-text>
            <button-text slot="filters-button">Show all my JIGs</button-text>
            <input-search slot="search-input"></input-search>
            ${mapToString(arrayCount(props.recentCount), recent)}
        </jig-gallery>
    `;
}

JigGallery.args = DEFAULT_ARGS;


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