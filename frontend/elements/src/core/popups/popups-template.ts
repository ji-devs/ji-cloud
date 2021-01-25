import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';


export type Color = "peach" | "green" ;
export type Size ="medium" | "large";


@customElement('template-popups')
export class _ extends LitElement {
  static get styles() {
    return [css`

div{
  position: relative;
}


  .medium{
    width: 576px;
    height: 352px;
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

 