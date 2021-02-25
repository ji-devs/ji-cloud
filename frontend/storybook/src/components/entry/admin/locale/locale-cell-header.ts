import "@elements/entry/admin/locale/locale-cell-header";
import {argsToAttrs} from "@utils/attributes";


export default {
  title: 'Entry/Admin/locale-cell-header',
}

interface Args {
    label: string,
    adminOnly: boolean,
}

const DEFAULT_ARGS: Args = {
    label: "Label",
    adminOnly: false,
}

export const LocaleCellHeader = (props?: Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <locale-cell-header ${argsToAttrs(props)}>
            <select slot="actions" multiple="" style="width: 100px;">
                <option value="sec1">sec1</option>
                <option value="sec2">sec2</option>
                <option value="sec3">sec3</option>
                <option value="sec4">sec4</option>
            </select>
            <locale-sort-button slot="actions"></locale-sort-button>
        </locale-cell-header>
    `
}

LocaleCellHeader.args = DEFAULT_ARGS;
