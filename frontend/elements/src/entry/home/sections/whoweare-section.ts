import { MEDIA_UI } from '@utils/path';
import "@elements/core/lists/column-list";
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
 .listwspace{
  display:block;
  margin-top:16px;
}
  

    `];
  }


 
 

  render() {

 

    const STR_TITLE="Who we are";
    const STR_LINE1="  Jewish Interactive is a registered 501(c)(3)  ";
    const STR_LINE2="in the US with tax ID 46-1331618  ";
    const STR_LINE3="The Jewish interactive Educational Trust is a  ";
    const STR_LINE4="  Section 18A (1)(a) in South Africa  ";
    const STR_LINE5="   (Registration IT36/2012) (PBO 930 038 343) ";
    const STR_LINE6=" Jewish Interactive is a registered charity  ";
    const STR_LINE7="in the UK (Charity Number 1151408)  ";
    const STR_DONATE="Donate";

    return html`
    <main>
    <h3>${STR_TITLE}</h3>
    <ul>
    <column-list text_line="${STR_LINE1}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_LINE2}" color="white" class="list"></column-list><br>
 
    <column-list text_line="${STR_LINE3}" color="white" class="listwspace"></column-list>
    <column-list text_line="${STR_LINE4}" color="white" class="list"></column-list>
    <column-list text_line="${STR_LINE5}" color="white" class="list"></column-list><br>
 
    <column-list text_line="${STR_LINE6}" color="white" class="listwspace"></column-list>
    <column-list text_line="${STR_LINE7}" color="white" class="list"></column-list>
    
    </slot>

    </ul>
    <button-rect class="Donate" size="large"  color="blue"  bold="true" imglefthidden="true" imgrighthidden="true">${STR_DONATE} </button-rect>
    </main>
  `;
  }
}