import "@elements/entry/admin/jig_label_ui/jig-details";
import "@elements/core/inputs/composed/select/select";

export default {
  title: "Entry/Admin/Jig Details",
  component: "jig-details",
};

export const JigDetails = () => {
  return `
    <jig-details>
      <input-wrapper id="first-input" label="JIG's name">
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
    </jig-details>
  `
};
JigDetails.args = {}
