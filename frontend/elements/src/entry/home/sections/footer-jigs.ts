 import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/lists/column-list";
@customElement('footer-jigs')
export class _ extends LitElement {
  static get styles() {
    return [css`
    h3{
      font-family: Poppins;
      font-size: 20px;
      font-weight: 800;   
      color:#ffffff;
    
    }
    
   
   ul{
    list-style-type: none;
    margin:0;
    padding:0;
   }


  .list{
    display:block;
    margin-top:8px;
 }
  

    `];
  }

  render() {

    const STR_SUPPORT ="Support & FAQ";
    const STR_TOUR = "Quick tour";
    const STR_TUTORIALS = "JI Tutorials";
    const STR_WEBINARS = "Online webinars";
    const STR_ACCESSIBILITY = "Accessibility"

    return html`
    <main>
    <h3>JIGs</h3>
      <ul>
        <column-list text_line="${STR_SUPPORT}" color="white" class="list" ></column-list>
        <column-list text_line="${STR_TOUR}" color="white" class="list" ></column-list>
        <column-list text_line="${STR_TUTORIALS}" color="white" class="list" ></column-list>
        <column-list text_line="${STR_WEBINARS}" color="white" class="list" ></column-list>
        <column-list text_line="${STR_ACCESSIBILITY}" color="white" class="list" ></column-list>

      </ul>
    
    </main>
  `;
  }
}