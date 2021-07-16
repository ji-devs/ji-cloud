import { LitElement, html, css, customElement, property} from 'lit-element';
import {loadAllFonts, ThemeId} from "@elements/_themes/themes";

export type Variant = "h1" | "h2" | "p1" | "p2";

@customElement('mock-text-example')
export class _ extends LitElement {
  static get styles() {
      return [css`
      `];
  }

  @property()
  variant:Variant = "h1";

  @property()
  theme:ThemeId = "blank";

  @property()
  text:string = "hello world";

	@property({type: Boolean})
	fontsLoaded:boolean = false;
	
	connectedCallback() {
		super.connectedCallback();

		loadAllFonts().then(() => {
			this.fontsLoaded = true;
		});
	}
 
 
  render() {
	const {fontsLoaded, text, theme, variant} = this;

	if(fontsLoaded) {
		let style = `font-family: var(--theme-${theme as string}-${variant}-font-family);`;
		style += ` font-size: var(--theme-${theme as string}-${variant}-font-size);`;
		style += ` color: var(--theme-${theme as string}-${variant}-color);`;


		return html`<div style="${style}">${text}</div>`
	} else {
		return html`<div>Loading fonts...</div>`;
	}
  }
}
