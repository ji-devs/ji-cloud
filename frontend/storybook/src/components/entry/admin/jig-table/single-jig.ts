import { JigData } from "./types";

import "@elements/entry/admin/jig_label_ui/single-jig";

const jig: JigData = {
  jig_name: "Hebrew Letters",
  author: "Michael Wikes",
  author_badge: "JI Team",
  date: "Aug. 5, 2020",
  language: "English (American)",
  curators: "Anat (13.7.21)",
};

export default {
  title: "Entry/Admin/Single Jig",
  component: "single-jig",
};

export const SingleJig = ({jig}) => {
  console.log({jig})
  return `
  <single-jig>
    <span slot="jig-name">${jig.jig_name}</span>
    <span slot="author">${jig.author}</span>
    <span slot="author-badge">${jig.author_badge}</span>
    <span slot="date">${jig.date}</span>
    <span slot="language">${jig.language}</span>
    <span slot="curators">${jig.curators}</span>
  </single-jig>
`
};
SingleJig.args = {
  jig
}
