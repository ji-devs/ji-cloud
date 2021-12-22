import {jigs} from './story-data';

import "@elements/entry/admin/jig_label_ui/single-jig";

const jig = jigs[0];

export default {
  title: "Entry/Admin/Single Jig",
  component: "single-jig",
};

export const SingleJig = ({jig}) => {
  return `
  <single-jig>
    <span slot="jig-name">${jig.jig_name}</span>
    <span slot="author">${jig.author}</span>
    <span slot="author-badge">${jig.author_badge}</span>
    <span slot="date">${jig.date}</span>
    <span slot="language">${jig.language}</span>
    <span slot="curators">${jig.curators}</span>
    <span slot="age-ranges">${jig.age_ranges}</span>
    <span slot="affiliations">${jig.affiliations}</span>
  </single-jig>
`
};
SingleJig.args = {
  jig
}
