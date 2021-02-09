import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('search-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
        .jig-section{
            width:1656px;
            margin-left:auto;
            margin-right:auto;
        }
        .section{
            display:grid;
            grid-template-columns:repeat(4, 1fr)
        }
    `];
  }

  render() {

    const {} = this;
   
    return html`    
    <div class="jig-section">
    <title-ji size="title-large" color="darkblue"></title-ji>
        <div class="section">
            <slot name="card"></slot>
        </div>
          
    </div>

  `;
  }
}