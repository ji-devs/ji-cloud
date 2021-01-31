import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';


export type Color = "peach" | "green" ;
export type Size ="medium" | "large";


@customElement('popup-container')
export class _ extends LitElement {
  static get styles() {
    return [css`

div{
  position: relative;
  border-radius:25px;
}


  .medium{
    width: 576px;
    height: 352px;
  }
<<<<<<< HEAD
.peach::before{
  content: "";
  position: absolute;
  bottom: 100%;
  left: 75%;
  margin-left: -5px;
  border-width: 20px;
  border-style: solid;
  border-color: transparent transparent #fff2cb transparent;
}

.green::before{
  content: "";
  position: absolute;
  bottom: 100%;
  left: 75%;
  margin-left: -5px;
  border-width: 20px;
  border-style: solid;
  border-color: transparent transparent #c4ead1 transparent;
=======
div::before{
  content: "";
  position: absolute;
  bottom: 100%;
  left: 50%;
  margin-left: -5px;
  border-width: 5px;
  border-style: solid;
  border-color: transparent transparent black transparent;
>>>>>>> naomi/master
}
 

  .large{
    width: 760px;
    height: 462px;
  }

  .peach{
    background-color:#fff2cb; 

  }

  .green{

    background-color:#c4ead1; 

  }

  img-ui{
    top:20px;
     right:25px;
      display:block;
      position: absolute;

  }
    `]
  }


  @property()
  color:Color = "green"; 
  @property()
  size:Size = "medium"; 


  render() {

    const {color,size} = this;
    const classes = classMap({ 
      [size]: true,
      [color]: true,
     
     
    });
    return html`
     <div class="${classes}">
     <img-ui path="Icn_Delete_32.png"></img-ui>
      <slot ></slot>
      </div>
        
  `;
  }
}

 