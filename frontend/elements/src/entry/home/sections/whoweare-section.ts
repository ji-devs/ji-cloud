import { MEDIA_UI } from '@utils/path';
import "@elements/entry/home/TOSORT/column-list";
import "@elements/entry/home/social-networks";
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('whoweare-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    h3{
      font-family: Poppins;
      font-size: 20px;
      font-weight: 800;   
      color:#ffffff;
    
    }
   main{
     
   }
   
 .Donate{
    margin-top:91px;
    display:block;

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

    const {} = this;

    return html`
    <main>
    <h3> Who we are </h3>
    <ul>
    <column-list text_line=" Jewish Interactive is a registered 501(c)(3)" color="white" class="list" ></column-list>
    <column-list text_line="in the US with tax ID 46-1331618  " color="white" class="list"></column-list><br>
 
    <column-list text_line="The Jewish interactive Educational Trust is a" color="white" class="list"></column-list>
    <column-list text_line="  Section 18A (1)(a) in South Africa " color="white" class="list"></column-list>
    <column-list text_line="(Registration IT36/2012) (PBO 930 038 343)" color="white" class="list"></column-list><br>
 
    <column-list text_line=" Jewish Interactive is a registered charity " color="white" class="list"></column-list>
    <column-list text_line="in the UK (Charity Number 1151408) " color="white" class="list"></column-list>  
    
    </slot>

    </ul>
    <button-rect class="Donate" size="large"  color="blue"  bold="true" imglefthidden="true" imgrighthidden="true">Donate </button-rect>
    </main>
  `;
  }
}