import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/buttons/circle";
import "@elements/core/buttons/arrow-button";
 import { arrayIndex } from '@utils/array';

export interface Item {
  imgPath: string,
  title: string,
  subtitle: string,
  body: string,
}

@customElement('moving-card')
export class _ extends LitElement {
  static get styles() {
    return [css`

   main{
    width: 822px;
    height: 403px;
     
   }
 
  
   .wrapper{
     display:flex;
   }

   button-circle{
     margin-left:-50px;
    }

   .nav{
      margin-left:320px;
      margin-top:30px;
      display:flex;

   }

.title{
  font-weight: bold;
  color: #fed657;
  font-size: 40px;


}

.subtitle{
  color:#383838;
  font-weight: 500;
  font-size: 18px;

}
.body{
  width:382px;
  margin-right:30px;
  margin-top:60px;

}
.content-card{
  display:flex;

}

arrow-button{
  margin-top:240px;
    margin-right:35px;

}

img-ui{
  margin-right:40px;
  margin-left:32px;
  margin-top:175px;
 }
    `];
  }

  @property({ type: Number })
  nItems: number= 5;

  @property({ type: Array })
  items: Array<Item> = [];

  @property({ type: Number })
  activeIndex: number = 0;

  @property()
  rightArrowState: boolean = false;

  @property()
  leftArrowState: boolean = false;





  render() {

    const { activeIndex, nItems, items } = this;
    const { title, subtitle, body, imgPath } = items[activeIndex];

    const ablerightarrow = activeIndex >= nItems-1 ?"disable":"able";

    const ableleftarrow = activeIndex === 0 ? "disable":"able";
    return html`
    <main>
      <div class="wrapper"> 

      <arrow-button  color="pink" direction="left"></arrow-button>

      <div class="content-card">
      <img-ui path="${imgPath}"></img-ui>
      <div class="body">
       <h2 class="title">${title}</h2>
        <h3 class="subtitle">${subtitle}</h3>
        <column-list text_line="${body}"></column-list>
        </div>
         </div>
        <arrow-button  color="yellow"></arrow-button>
         </div>
          <div class="nav">
          ${makeCircles(activeIndex, nItems)}
     </div>
    </main>
  `;
  }
}


const makeCircles = (activeIndex: number, nItems: number) => {
   return arrayIndex(nItems)
  .map((  ) => html`
  
<button-circle color="pink" size="small"></button-circle>
 
`)
}