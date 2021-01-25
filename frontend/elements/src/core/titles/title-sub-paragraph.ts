import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';

export type Color = "yellow" | "black" | "white";
export type FontSize = "small" | "medium";

@customElement('title-sub-paragraph')
export class _ extends LitElement {
  static get styles() {
    return [css`


  .yellow {
    color: #fed657;
  }
  .black{
    color:#383838;
  
  }
  .white{
    color:#ffffff;
  }

  .medium{
    font-size: 40px;

  }

  .small{
    font-size: 18px;
  }
h2{
    font-weight: bold;

}
h3{
    font-weight: 500;

}



  .inside{
    // display: grid;
    // grid-template-columns: 1fr;
    // grid-template-rows: 1fr 1fr 1fr;
  }
    `]
  }


 
  @property()
  colorsubtitle:Color = "black"; 
  @property()
  title:string = ""; 
  @property()
  sizesubtitle:FontSize = "small"; 
  @property()
  subtitle:string = ""; 
  @property()
  color:Color = "black"; 
  @property()
  size:FontSize = "small"; 

  render() {

    const {size,color, title,colorsubtitle,sizesubtitle,subtitle} = this;

    const classes = classMap({ 
      [size]: true,
      [color]: true,
     
     
    });

  
    return html`
     <div class="wrapper">
        <div class="inside">
          <h2 class="${classes}">${title}</h2>
          <h3 class="${colorsubtitle} ${sizesubtitle} ">${subtitle}</h3>

         <slot name="line"></slot>
        </div>
      </div>
        
  `;
  }
}

 