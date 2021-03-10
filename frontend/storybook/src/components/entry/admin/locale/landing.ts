import "@elements/entry/admin/locale/landing";
import "@elements/entry/admin/locale/locale-row";
import "@elements/entry/admin/locale/locale-cell";
import "@elements/entry/admin/locale/locale-cell-header";
import "@elements/entry/admin/locale/locale-actions-wrapper";
import "@elements/entry/admin/locale/locale-hover-link";
import "@elements/entry/admin/locale/locale-sort-button";
import "@elements/core/buttons/rectangle";
import {argsToAttrs} from "@utils/attributes";


export default {
  title: 'Entry/Admin/Locale/pages',
}

interface Args {
    saving: boolean,
    sortOrder: 'asc' | 'desc',
}

const DEFAULT_ARGS: Args = {
    saving: false,
    sortOrder: 'asc',
}

export const Landing = (props?: Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
<locale-page ${argsToAttrs(props)} sortorder="asc" columns='[["ID", true], ["Section", true], ["Item Kind", true], ["English", true], ["Hebrew", false], ["Status", true], ["Zeplin reference", true], ["Comments", true], ["App", true], ["Element", true], ["Mock", true], ["Actions", true]]'>
    <select slot="bundles" multiple="">
        <option value="Publish">Publish</option>
        <option value="Poster">Poster</option>
        <option value="JIG">JIG</option>
        <option value="Memory game">Memory game</option>
    </select>
    <button-rect slot="add-entry" color="blue">Add a text</button-rect>
    <locale-select-columns slot="dialog-content">
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
    <locale-row slot="rows">
        <locale-cell-header label="ID"></locale-cell-header>
        <locale-cell-header label="Section">
            <select slot="actions" multiple="" style="width: 100px;">
                <option value="sec1">sec1</option>
                <option value="sec2">sec2</option>
                <option value="sec3">sec3</option>
                <option value="sec4">sec4</option>
            </select>
            <locale-sort-button slot="actions"></locale-sort-button>
        </locale-cell-header>
        <locale-cell-header label="Item Kind">
            <select slot="actions" multiple="" style="width: 100px;">
                <option value="Button">Button</option>
                <option value="Subheading">Subheading</option>
                <option value="hay">hay</option>
            </select>
            <locale-sort-button slot="actions" sorted=""></locale-sort-button>
        </locale-cell-header>
        <locale-cell-header label="English">
            <locale-sort-button slot="actions"></locale-sort-button>
        </locale-cell-header>
        <locale-cell-header label="Status">
            <select slot="actions" multiple>
                <option value="Approved">Approved</option>
                <option value="Discuss">Discuss</option>
                <option value="OnHold">OnHold</option>
            </select>
            <locale-sort-button slot="actions"></locale-sort-button>
        </locale-cell-header>
        <locale-cell-header label="Zeplin reference"></locale-cell-header>
        <locale-cell-header label="Comments">
            <locale-sort-button slot="actions"></locale-sort-button>
        </locale-cell-header>
        <locale-cell-header label="App"></locale-cell-header>
        <locale-cell-header label="Element"></locale-cell-header>
        <locale-cell-header label="Mock"></locale-cell-header>
        <locale-cell-header></locale-cell-header>
    </locale-row>
    <locale-row slot="rows">
        <locale-cell>
            <input value="somthing">
        </locale-cell>
        <locale-cell>
            <input value="somthing" list="sections">
        </locale-cell>
        <locale-cell>
            <input value="somthing" list="item-kinds">
        </locale-cell>
        <locale-cell>
            <textarea>Hello world</textarea>
        </locale-cell>
        <locale-cell>
            <select>
                <option value="Approved">Approved</option>
                <option value="Discuss">Discuss</option>
                <option value="OnHold">OnHold</option>
            </select>
        </locale-cell>
        <locale-cell>
            <locale-hover-link>
                <input value="somthing" type="url">
            </locale-hover-link>
        </locale-cell>
        <locale-cell>
            <input value="somthing">
        </locale-cell>
        <locale-cell>
            <input value="somthing" checked type="checkbox">
        </locale-cell>
        <locale-cell>
            <input value="somthing" checked type="checkbox">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <locale-actions-wrapper>
                <button-text slot="first">Clone</button-text>
                <button-text slot="second">Delete</button-text>
            </locale-actions-wrapper>
        </locale-cell>
    </locale-row>
    <locale-row slot="rows">
        <locale-cell>
            <input value="somthing">
        </locale-cell>
        <locale-cell>
            <input value="somthing" list="sections">
        </locale-cell>
        <locale-cell>
            <input value="somthing" list="item-kinds">
        </locale-cell>
        <locale-cell>
            <textarea>{$userName} {$photoCount -&gt;
    [one] added a new photo
   *[other] added {$photoCount} new photos
} to {$userGender -&gt;
    [male] his stream
    [female] her stream
   *[other] their stream
}.
</textarea>
        </locale-cell>
        <locale-cell>
            <select>
                <option value="Approved">Approved</option>
                <option value="Discuss">Discuss</option>
                <option value="OnHold">OnHold</option>
            </select>
        </locale-cell>
        <locale-cell>
            <locale-hover-link>
                <input value="somthing" type="url">
            </locale-hover-link>
        </locale-cell>
        <locale-cell>
            <input value="somthing">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <locale-actions-wrapper>
                <button-text slot="first">Clone</button-text>
                <button-text slot="second">Delete</button-text>
            </locale-actions-wrapper>
        </locale-cell>
    </locale-row>
    <locale-row slot="rows">
        <locale-cell>
            <input value="somthing">
        </locale-cell>
        <locale-cell>
            <input value="somthing" list="sections">
        </locale-cell>
        <locale-cell>
            <input value="somthing" list="item-kinds">
        </locale-cell>
        <locale-cell>
            <textarea>Hello world</textarea>
        </locale-cell>
        <locale-cell>
            <select>
                <option value="Approved">Approved</option>
                <option value="Discuss">Discuss</option>
                <option value="OnHold">OnHold</option>
            </select>
        </locale-cell>
        <locale-cell>
            <locale-hover-link>
                <input value="somthing" type="url">
            </locale-hover-link>
        </locale-cell>
        <locale-cell>
            <input value="somthing">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <locale-actions-wrapper>
                <button-text slot="first">Clone</button-text>
                <button-text slot="second">Delete</button-text>
            </locale-actions-wrapper>
        </locale-cell>
    </locale-row>
    <locale-row slot="rows">
        <locale-cell>
            <input value="somthing">
        </locale-cell>
        <locale-cell>
            <input value="somthing" list="sections">
        </locale-cell>
        <locale-cell>
            <input value="somthing" list="item-kinds">
        </locale-cell>
        <locale-cell>
            <textarea>Hello world</textarea>
        </locale-cell>
        <locale-cell>
            <select>
                <option value="Approved">Approved</option>
                <option value="Discuss">Discuss</option>
                <option value="OnHold">OnHold</option>
            </select>
        </locale-cell>
        <locale-cell>
            <locale-hover-link>
                <input value="somthing" type="url">
            </locale-hover-link>
        </locale-cell>
        <locale-cell>
            <input value="somthing">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <input value="somthing" type="checkbox">
        </locale-cell>
        <locale-cell>
            <locale-actions-wrapper>
                <button-text slot="first">Clone</button-text>
                <button-text slot="second">Delete</button-text>
            </locale-actions-wrapper>
        </locale-cell>
    </locale-row>
</locale-page>
    `
}

Landing.args = DEFAULT_ARGS;
