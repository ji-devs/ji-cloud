import { LitElement, html, css, customElement, property } from 'lit-element';
import {FluentBundle, FluentResource} from "@fluent/bundle";
import english from "../../../../../localization/fluent/entry/user/english.ftl";

const res = new FluentResource(english as any);

const bundle = new FluentBundle("en-US");
const errors = bundle.addResource(res);
if (errors.length) {
    throw new Error(JSON.stringify(errors));
    // Syntax errors are per-message and don't break the whole resource
}

@customElement('user-test-fluent')
export class _ extends LitElement {
  render() {
    const {value} = bundle.getMessage("test-element") as any;
    const str = bundle.formatPattern(value, {name: "World"});
    return html`
        <h1>${str}</h1>
   `; 
  }
}
