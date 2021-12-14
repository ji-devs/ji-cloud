import { JigData } from "./types";
import { jigs } from "./story-data";

import "@elements/entry/admin/jig_label_ui/jig-label-ui";
import "@elements/entry/admin/jig_label_ui/single-jig";

export default {
  title: "Entry/Admin/Jig Table UI",
  component: "jig-label-ui",
}

export const JigTable = ({ jigs }) => {
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
          </single-jig>
        `
      ).join('')}
    </jig-label-ui>
  `};
JigTable.args = {
  jigs,
};
