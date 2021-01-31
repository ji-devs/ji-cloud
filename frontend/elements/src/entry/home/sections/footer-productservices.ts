 import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/entry/home/TOSORT/column-list";
@customElement('footer-productservices')
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

    const STR_TEACHERS = "Teachers";
    const STR_PARENTS = "Parents";
    const STR_BITES ="JI Bites";
    const STR_PRIME = "JI Prime";
    const STR_TAP = "JI Tap";
    const STR_STUDIO = "JI Studio";
    const STR_COLLECTION = "The JI Collection";
    const STR_JSTREAM = "J-Stream";
    const STR_BLOG = "JI Blog";
    const STR_JOBS = "Jobs";


    return html`
    <main>
    <h3>Products & Services</h3>
    <ul>
    <column-list text_line="${STR_TEACHERS}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_PARENTS}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_BITES}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_PRIME}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_TAP}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_STUDIO}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_COLLECTION}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_JSTREAM}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_BLOG}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_JOBS}" color="white" class="list" ></column-list>

    </ul>
    
    </main>
  `;
  }
}