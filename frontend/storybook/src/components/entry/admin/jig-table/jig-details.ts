import "@elements/entry/admin/jig_label_ui/jig-details";
import "@elements/core/inputs/composed/select/select";

export default {
  title: "Entry/Admin/Jig Details",
  component: "jig-details",
};

export const JigDetails = () => {
  return `
    <jig-details>
      <input-select label="Input Select">
        <input-select-option>Option 1</input-select-option>
      </input-select>
      <p>Test</p>
    </jig-details>
  `
};
JigDetails.args = {}
