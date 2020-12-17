import { LitElement, html, css, customElement, property} from 'lit-element';
import {nothing} from "lit-html";

export class BaseText extends LitElement {
  static get styles() {
    return [css`
        .bold{
            font-weight: 600;
        }
        
    `]
  }
}