import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';


@customElement('selected-dropdown')
export class _ extends LitElement {

  static get styles() {
    return [css`
main{
    width: 272px;
    height:372px;
    border-radius: 14px;
    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
    border: solid 1px #eaebef;
    background-color: #f8f9fd;
    padding:8px 9px;

}
::slotted([slot=list]) {
    height: 284px;
    overflow:auto;
}
slot[name="list"]::slotted(input) {
    background-color:red !important;
}
    `];
  }

@property()
label: string = "";

  render() {
    return html`
    <main>
        <slot name="search"></slot>
        <slot name="list"></slot>
    </main>
  `;
  }
}