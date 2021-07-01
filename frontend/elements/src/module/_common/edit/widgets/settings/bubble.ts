import { LitElement, html, svg, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const TRIANGLE_WIDTH = 18;
const TRIANGLE_HEIGHT = 10;
const OUTLINE_SIZE = 3;


@customElement('module-settings-bubble')
export class _ extends LitElement {
  static get styles() {
      return [css`
	:host {
		position: relative;
		top: ${TRIANGLE_HEIGHT}px;
		left: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		transform: translateX(-50%);
	}

	.rect {
		display: flex;
		align-items: center;
		border-radius: 12px;
		padding: 12px;
		box-shadow: 0px 3px 6px rgba(0, 0, 0, 0.08);
		color: var(--dark-gray-6);
		background: #ffe1a7;
		border: ${OUTLINE_SIZE}px solid #fdd994;
	}


	.tri, .tri-repaint {
		position: absolute;
		top: -${TRIANGLE_HEIGHT}px;
	}

	.tri {
		fill: #ffe1a7;
	}
	.tri-repaint {
		fill-opacity: 0;
	}

	.tri path {
		stroke-width: ${OUTLINE_SIZE};
		stroke-linejoin: round;
		stroke-linecap:round;
	}

	.tri-repaint path {
		stroke-width: ${OUTLINE_SIZE};
		stroke-linejoin: round;
		stroke-linecap: round;
	}
	.tri path {
		stroke: #ffe1a7;
	}
	.tri-repaint path {
		stroke: #fdd994;
	}


    `];
  }

  render() {

      return html`
      	${renderSvgArrow()}
	<div class="rect"><slot></slot></div>	
      `
  }
}

	//<img-ui path="module/_common/edit/widgets/sidebar/settings/bubble/arrow.svg"></img-ui>
function renderSvgArrow() {
	const boxWidth = TRIANGLE_WIDTH + (OUTLINE_SIZE * 2); 
	const boxHeight = TRIANGLE_HEIGHT + (OUTLINE_SIZE * 2);

	const left = OUTLINE_SIZE;
	const right = OUTLINE_SIZE + TRIANGLE_WIDTH;
	const middle = OUTLINE_SIZE + (TRIANGLE_WIDTH/ 2); 
	const bottom = OUTLINE_SIZE + TRIANGLE_HEIGHT;
	const top = OUTLINE_SIZE;

	//First draw the triangle with no outline
	//then draw the outlines on right and left sides
	//then fill in the small gap left by the hole due to round edges
	return svg`
		<svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri" width='${boxWidth}' height='${boxHeight}'>
    			<path d="M ${left},${bottom} ${middle},${top} ${right},${bottom} z"/>
  		</svg>
		  <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
    			<path d="M ${left},${bottom - (OUTLINE_SIZE/2)} ${middle},${top} z"/>
  		   </svg>
		  <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
    			<path d="M 0,${bottom - (OUTLINE_SIZE/2)} ${left},${bottom - (OUTLINE_SIZE/2)} z"/>
  		   </svg>
		  <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
    			<path d="M ${right},${bottom - (OUTLINE_SIZE/2)} ${middle},${top} z"/>
  		   </svg>
		  <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
    			<path d="M ${boxWidth},${bottom - (OUTLINE_SIZE/2)} ${right},${bottom - (OUTLINE_SIZE/2)} z"/>
  		   </svg>
		  `
}
/*
		     */