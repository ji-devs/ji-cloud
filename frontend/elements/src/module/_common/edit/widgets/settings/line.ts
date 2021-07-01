import { LitElement, html, svg, css, customElement, property } from "lit-element";
import {nothing} from "lit-html";
import { classMap } from 'lit-html/directives/class-map';
import "@elements/core/images/ui";

export type Kind = "card-view"
	| "game-display"
	| "rounds"
	| "time-limit"
	| "attempts"
	| "score";

const STR_LABEL:Record<Kind, string> = {
	"card-view": "Select how the player will view the cards",
	"game-display": "How to display your game?",
	"rounds": "How many slides / questions?",
	"time-limit": "Would you like to set time limit?",
	"attempts": "How many attempts?",
	"score": "Would you like to include score?"
};

@customElement("module-settings-line")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        :host {
		display: grid; 
  		grid-template-columns: 123px 1fr; 
  		gap: 0px 70px; 
		  width: 491px;
        }


	.label {
		margin-top: 11px;
		font-size: 16px;
		font-weight: 500;
		line-height: 1.25;
		text-align: left;
		color: var(--dark-gray-4);
	}

	.options {
		display: grid; 
  		grid-template-columns: 64px 64px; 
  		gap: 0px 52px; 
	}

      `,
    ];
  }

  @property()
  kind: Kind = "card-view";

  @property({type: Boolean})
  borderTop: boolean = false;

  render() {
    const { kind} = this;


    const label = STR_LABEL[kind];
   
    return html`
	<div class="label">${label}</div>
	<div class="options"><slot></slot></div>
    `;
  }
}