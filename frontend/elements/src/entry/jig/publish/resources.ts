import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/variants/title-w-icon";
@customElement('resources-column')
export class _ extends LitElement {
  static get styles() {
    return [css`

    
    `];
  }


  @property({type:Boolean})
  uploaded: boolean = false;
  

  render() {

    const {uploaded} = this;
    
    return html`
    <div>
        <title-wicon title="Add lesson plan" path="Icn_Add.svg"  .uploaded=${uploaded}></title-wicon>
        <title-wicon title="Add Curriculum" path="Icn_Add.svg" .uploaded=${uploaded}></title-wicon>
        <title-wicon title="Add activity ideas" path="Icn_Add.svg" .uploaded=${uploaded}></title-wicon>
        <title-wicon title="Add link" path="Icn_Add.svg" .uploaded=${uploaded}></title-wicon>
    </div>
  `;
  }
}