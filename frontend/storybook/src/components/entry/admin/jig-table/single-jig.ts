import { Meta, Story } from "@storybook/web-components";
import { html } from "lit-element";
import { JigData } from "./types";

const jig: JigData = {
  jig_name: "Hebrew Letters",
  author: "Michael Wikes",
  author_badge: "JI Team",
  date: "Aug. 5, 2020",
  language: "English (American)",
  curators: "Anat (13.7.21)",
};

import "../single-jig";

export default {
  title: "Single Jig",
  component: "single-jig",
} as Meta;

const Template: Story = ({ jig }) => html`
  <single-jig>
    <span slot="jig-name">${jig.jig_name}</span>
    <span slot="author">${jig.author}</span>
    <span slot="author-badge">${jig.author_badge}</span>
    <span slot="date">${jig.date}</span>
    <span slot="language">${jig.language}</span>
    <span slot="curators">${jig.curators}</span>
  </single-jig>
`;

export const Primary = Template.bind({});
Primary.args = {
  jig,
};
