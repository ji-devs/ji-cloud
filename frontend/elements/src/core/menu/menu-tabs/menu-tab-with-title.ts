import { LitElement, html, css, customElement, property } from "lit-element";
import "./menu-tab";
import "./menu-tab-title";
import { TitleKind } from "./menu-tab-title";

@customElement("menu-tab-with-title")
export class _ extends LitElement {
  static get styles() {
    return [css`
      :host([disabled]) {
        pointer-events: none;
      }
    `];
  }

  @property({ type: Boolean })
  active: boolean = false;

  @property()
  kind: TitleKind = "";

  @property({ type: Boolean, reflect: true })
  disabled: boolean = false;

  render() {
    const { active, kind, disabled} = this;

    return html`
    	<menu-tab .active=${active} .disabled=${disabled}>
	    <menu-tab-title .kind=${kind} .disabled=${disabled} .active=${active}></menu-tab-title>
    	</menu-tab>
	`;
  }
}
