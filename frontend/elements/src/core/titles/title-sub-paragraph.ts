import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('title-sub-paragraph')
export class _ extends LitElement {
  static get styles() {
    return [css`


  .yellow {
    color: #fed657    ;
  }
  .black{
    #383838
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
  colortitle:string = ""; 
  @property()
  colorsubtitle:string = ""; 
  @property()
  title:string = ""; 
  @property()
  sizetitle:string = ""; 
  @property()
  sizesubtitle:string = ""; 
 
  @property()
  subtitle:string = ""; 
 

  render() {

    const {colortitle, colorsubtitle,sizetitle,sizesubtitle,title,subtitle} = this;

    return html`
     <div class="wrapper">
        <div class="inside">
          <h2 class="${colortitle} ${sizetitle} ">${title}</h2>
          <h3 class="${colorsubtitle} ${sizesubtitle} ">${subtitle}</h3>

         <slot name="line"></slot>
        </div>
      </div>
        
  `;
  }
}

 