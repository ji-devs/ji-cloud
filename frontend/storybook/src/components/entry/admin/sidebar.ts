import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/admin/sidebar/container";
import "@elements/entry/admin/sidebar/item";
import {ID} from "@elements/entry/admin/sidebar/item";
import { mapToString } from "@utils/array";

const ids:Array<ID> = [
	"image-add",
	"image-tags",
	"image-search",
	"jigs",
	"category",
	"locale",
];

export default {
    title: "Entry/Admin/Sidebar"
}

interface Args {
    selected: ID | "",
}

const DEFAULT_ARGS:Args = {
    selected: "image-tags"
}

export const Sidebar = (props?:Partial<Args>) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {selected, ...adminProps} = props;

    return `<admin-sidebar ${argsToAttrs(adminProps)}>
        ${
            mapToString(ids, (id, idx) => {
                const itemProps = {
                    id,
                    selected: selected === id,
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
            options: ["", ...ids]
        }
    }
}
