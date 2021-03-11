import "@elements/core/buttons/text";
import "@elements/entry/admin/locale/locale-select-columns";
import "@elements/entry/admin/locale/locale-select-columns-item";
import {argsToAttrs} from "@utils/attributes";


export default {
  title: 'Entry/Admin/Locale',
}

interface Args {
}

const DEFAULT_ARGS: Args = {
}

export const LocaleSelectColumns = (props?: Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
    <dialog open>
        <locale-select-columns>
            <locale-select-columns-item slot="hidden-columns">Column A</locale-select-columns-item>
            <locale-select-columns-item slot="hidden-columns">Column B</locale-select-columns-item>
            <locale-select-columns-item slot="hidden-columns">Column C</locale-select-columns-item>
            <locale-select-columns-item slot="hidden-columns">Column D</locale-select-columns-item>

            <button-text slot="move-actions">⇨</button-text>
            <button-text slot="move-actions">⇦</button-text>


            <locale-select-columns-item slot="visible-columns" active>Column E (active)</locale-select-columns-item>
            <locale-select-columns-item slot="visible-columns">Column F</locale-select-columns-item>
            <locale-select-columns-item slot="visible-columns">Column G</locale-select-columns-item>

            <button-text slot="sort-actions">⇧</button-text>
            <button-text slot="sort-actions">⇩</button-text>

        </locale-select-columns>
    </dialog>
    `
}

LocaleSelectColumns.args = DEFAULT_ARGS;


