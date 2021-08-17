import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/admin/sidebar/container";
import "@elements/entry/admin/sidebar/item";
import {SECTION} from "@elements/entry/admin/sidebar/item";
import { mapToString } from "@utils/array";

const sections:Array<SECTION> = [
	"image-add",
	"image-tags",
	"image-search",
	"jig",
	"category",
	"locale",
];

export default {
    title: "Entry/Admin/Sidebar"
}

interface Args {
    selected: SECTION | "",
}

const DEFAULT_ARGS:Args = {
    selected: "image-tags"
}

export const Sidebar = (props?:Partial<Args>) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {selected, ...adminProps} = props;

    return `<admin-sidebar ${argsToAttrs(adminProps)}>
        ${
            mapToString(sections, (section, idx) => {
                const itemProps = {
                    section,
                    selected: selected === section,
                    locked: idx % 2 == 0
                };
                return `<admin-sidebar-item ${argsToAttrs(itemProps)}></admin-sidebar-item>`
            })
        }
    </admin-sidebar>`;
}

Sidebar.args = DEFAULT_ARGS;
Sidebar.argTypes = {
    selected: {
        control: {
            type: 'inline-radio',
            options: ["", ...sections]
        }
    }
}
