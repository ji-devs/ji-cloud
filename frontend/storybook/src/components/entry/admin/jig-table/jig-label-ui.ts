import { JigData } from "./types";
import { jigs } from "./story-data";

import "@elements/entry/admin/jig_label_ui/jig-label-ui";
import "@elements/entry/admin/jig_label_ui/single-jig";

export default {
  title: "Entry/Admin/Jig Table UI",
  component: "jig-label-ui",
}

export const JigTableUI = ({ jigs }) => {
  return `
    <jig-label-ui>
      ${jigs.map(
        (jig: JigData) => `
          <single-jig>
            <span slot="jig-name">${jig.jig_name}</span>
            <span slot="author">${jig.author}</span>
            <span slot="author-badge">${jig.author_badge}</span>
            <span slot="date">${jig.date}</span>
            <span slot="language">${jig.language}</span>
            <span slot="curators">${jig.curators}</span>
            <span slot="age-ranges">${jig.age_ranges}</span>
            <span slot="affiliations">${jig.affiliations}</span>
            <jig-details slot="jig-details">
              <div slot="buttons">
                <button-rect kind="text" color="blue">Cancel</button-rect>
                <button-rect kind="outline" color="blue">Save Changes</button-rect>
              </div>
              <div slot="inputs">
                <input-wrapper label="JIG's name">
                  <input type="text" value="">
                </input-wrapper>
                <input-wrapper label="Author name">
                  <input type="text" value="">
                </input-wrapper>
                <input-select label="Instruction Language">
                  <input-select-option>English</input-select-option>
                  <input-select-option>Spanish</input-select-option>
                  <input-select-option>Hebrew</input-select-option>
                  <input-select-option>French</input-select-option>
                  <input-select-option>Italian</input-select-option>
                </input-select>
                <input-select label="Suitable for age">
                  <input-select-option>All ages</input-select-option>
                  <input-select-option>No ages</input-select-option>
                </input-select>
                <input-select label="Affiliation">
                  <input-select-option>Affiliation 1</input-select-option>
                  <input-select-option>Affiliation 2</input-select-option>
                  <input-select-option>Affiliation 3</input-select-option>
                </input-select>
                <input-wrapper label="JIG teacher's description">
                  <textarea id="description" rows="6" value=""></textarea>
                </input-wrapper>
                <input-wrapper label="Additional keywords">
                  <textarea rows="6" value=""></textarea>
                </input-wrapper>
              </div>
            </jig-details>
          </single-jig>
        `
      ).join('')}
    </jig-label-ui>
  `};
JigTableUI.args = {
  jigs,
};
