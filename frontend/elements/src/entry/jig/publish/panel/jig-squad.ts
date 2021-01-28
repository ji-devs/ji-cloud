 import { LitElement, html, css, customElement, property } from 'lit-element';
 

@customElement('jig-squad')
export class _ extends LitElement {
  static get styles() {
    return [css`
    
    
   main{
    width: 416px;
    height: 168px;
  
      
   }
  .wrapper{
    display:flex;
  }
.feet{
    margin-left:48%;
    margin-top:-78px;

    display:block;
}
 ::slotted([slot=subtitle]){
  margin-left:10px;
  display:block;
}

.icn-menu{
    margin-left:360px;
     margin-top:-180px;
    display:block;
}

    `];
  }



  @property()
  pathfeet:string = ""; 



 

  render() {

    const {pathfeet } = this;

     


    return html`
    <main>
    <div class="wrapper">
    <slot name="side-bar-number"></slot>

    <slot name="jiggling-body"></slot>
    </div>
     <img-ui class="feet" path="${pathfeet}"></img-ui>
     <img-ui class="icn-menu" path="Icn_Menu_JIG.svg"></img-ui>
    </main>
  `;
  }
}