import "@elements/entry/admin/locale/landing";
import "@elements/entry/admin/locale/locale-row";
import "@elements/entry/admin/locale/locale-cell";
import "@elements/entry/admin/locale/locale-cell-header";


export default {
  title: 'Entry/Admin/Locale',
}

export const Landing = () => {
    return `
<locale-page
    bundles='[["JIG", true], ["Memory game", true], ["Publish", false], ["Poster", true]]'
    columns='[["ID", true], ["Section", true], ["Item Kind", true], ["English", true], ["Hebrew", false], ["Status", true], ["Zeplin reference", true], ["Comments", true], ["App", true], ["Element", true], ["Mock", true], ["Actions", true]]'
>
    <locale-row slot="rows">
        <locale-cell-header
            label="ID"
        ></locale-cell-header>
        <locale-cell-header
            label="Section"
            sortable
            filteroptions='[["sec1", true], ["sec2", true], ["sec3", true], ["sec4", true]]'
        ></locale-cell-header>
        <locale-cell-header
            label="Item Kind"
            sortable
            filteroptions='[["Button", true], ["Subheading", true], ["hay", true]]'
        ></locale-cell-header>
        <locale-cell-header
            label="English"
            sortable
        ></locale-cell-header>
        <locale-cell-header
            label="Status"
            sortable
            filteroptions='[["Approved", true], ["Discuss", true], ["On Hold", true]]'
        ></locale-cell-header>
        <locale-cell-header
            label="Zeplin reference"
        ></locale-cell-header>
        <locale-cell-header
            label="Comments"
            sortable
        ></locale-cell-header>
        <locale-cell-header
            label="App"
            adminOnly
        ></locale-cell-header>
        <locale-cell-header
            label="Element"
            adminOnly
        ></locale-cell-header>
        <locale-cell-header
            label="Mock"
            adminOnly
        ></locale-cell-header>
        <locale-cell-header
        ></locale-cell-header>
    </locale-row>
    <locale-row slot="rows">
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input list="sections">
        </locale-cell>
        <locale-cell>
            <input list="item-kinds">
        </locale-cell>
        <locale-cell>
            <textarea>Hello world</textarea>
        </locale-cell>
        <locale-cell>
            <select>
                <option value="Approved">Approved</option>
                <option value="Discuss">Discuss</option>
                <option value="On Hold">On Hold</option>
            </select>
        </locale-cell>
        <locale-cell class="zeplin-link-cell"><a target="_blank" class="zeplin-link" href=""></a><input type="url">
        </locale-cell>
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <div class="actions-wrapper">
                <button class="link-button">Clone</button>
                <span>|</span>
                <button class="link-button">Delete</button>
            </div>
        </div>
    </locale-row>
    <locale-row slot="rows">
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input list="sections">
        </locale-cell>
        <locale-cell>
            <input list="item-kinds">
        </locale-cell>
        <locale-cell>
            <textarea>Hello world</textarea>
        </locale-cell>
        <locale-cell>
            <select>
                <option value="Approved">Approved</option>
                <option value="Discuss">Discuss</option>
                <option value="On Hold">On Hold</option>
            </select>
        </locale-cell>
        <locale-cell class="ftl-cell zeplin-link-cell">
            <a target="_blank" class="zeplin-link" href=""></a>
            <input type="url">
        </locale-cell>
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>

            <div class="actions-wrapper">
                <button class="link-button">Clone</button>
                <span>|</span>
                <button class="link-button">Delete</button>
            </div>
        </div>
    </locale-row>
    <locale-row slot="rows">
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input list="sections">
        </locale-cell>
        <locale-cell>
            <input list="item-kinds">
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
                <option value="On Hold">On Hold</option>
            </select>
        </locale-cell>
        <locale-cell class="ftl-cell zeplin-link-cell">
            <a target="_blank" class="zeplin-link" href="https://google.com/">https://google.com/</a>
            <input type="url">
        </locale-cell>
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <div class="actions-wrapper">
                <button class="link-button">Clone</button>
                <span>|</span>
                <button class="link-button">Delete</button>
            </div>
        </div>
    </locale-row>
    <locale-row slot="rows">
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input list="sections">
        </locale-cell>
        <locale-cell>
            <input list="item-kinds">
        </locale-cell>
        <locale-cell>
            <textarea>Hello world</textarea>
        </locale-cell>
        <locale-cell>
            <select>
                <option value="Approved">Approved</option>
                <option value="Discuss">Discuss</option>
                <option value="On Hold">On Hold</option>
            </select>
        </locale-cell>
        <locale-cell class="zeplin-link-cell"><a target="_blank" class="zeplin-link" href=""></a><input type="url">
        </locale-cell>
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>

            <div class="actions-wrapper">
                <button class="link-button">Clone</button>
                <span>|</span>
                <button class="link-button">Delete</button>
            </div>
        </div>
    </locale-row>
    <locale-row slot="rows">
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input list="sections">
        </locale-cell>
        <locale-cell>
            <input list="item-kinds">
        </locale-cell>
        <locale-cell>
            <textarea>Hello world</textarea>
        </locale-cell>
        <locale-cell>
            <select>
                <option value="Approved">Approved</option>
                <option value="Discuss">Discuss</option>
                <option value="On Hold">On Hold</option>
            </select>
        </locale-cell>
        <locale-cell class="zeplin-link-cell"><a target="_blank" class="zeplin-link" href=""></a><input type="url">
        </locale-cell>
        <locale-cell>
            <input>
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>
            <input type="checkbox">
        </locale-cell>
        <locale-cell>

            <div class="actions-wrapper">
                <button class="link-button">Clone</button>
                <span>|</span>
                <button class="link-button">Delete</button>
            </div>
        </div>
    </locale-row>
</locale-page>
    `
}
