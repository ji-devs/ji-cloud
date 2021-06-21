import { LitElement, html, css, customElement, property } from "lit-element";
import "./menu-tab";
import "./menu-tab-title";
import { TitleKind } from "./menu-tab-title";

@customElement("menu-tab-with-title")
export class _ extends LitElement {
  static get styles() {
    return [css``];
  }

  @property({ type: Boolean })
  active: boolean = false;

  @property()
  kind: TitleKind = "";

  render() {
    const { active, kind} = this;

    return html`
    	<menu-tab .active=${active}>
	    <menu-tab-title .kind=${kind} .active=${active}></menu-tab-title>
    	</menu-tab>
	`;
  }
}
