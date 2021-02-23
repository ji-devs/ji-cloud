import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import "@elements/core/titles/variants/horizontal-underlined-title";

    const STR_BACK = "Back";
    const STR_PAGE = "Page";
    const STR_OF = "of";
    const STR_NEXT ="Next";

@customElement('pagination-widget')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: flex;
                color: #4a4a4a;
            }
            .icon-btn {
                display: flex;
                align-items: center;
                opacity: 30%;
                cursor: default;
            }

            .icon-btn.active {
                cursor: pointer;
                opacity: 100%;
            }

            .icon-btn.prev > img-ui {
                margin-right: 10px;
            }
            .icon-btn.next > img-ui {
                margin-left: 10px;
            }
            
            .total:before { content: "\\00a0 "; }

            input {
                width: 24px;
                height: 24px;
                border: solid 1px #c4dbfe;
                background-color: #e6f0ff;
                color: #5590fc;
                margin:0 8px;
                outline: none;

            }

            .middle {
                margin: 0 55px;
            }
        `]
    }

    onPrev() {
        this.dispatchChange(this.page-1);
    }
    onNext() {
        this.dispatchChange(this.page+1);
    }

    onChange(evt:any) {
        this.updateSize();
        this.dispatchChange(evt.target.value);
    }

    updateSize() {
    }

    dispatchChange(target:number) {
        const {total, page} = this;

        if(target >= 1 && target <= total) {
            this.page = target;
            this.dispatchEvent(new CustomEvent("custom-change", {
                detail: { value: target.toString() },
            }))
        } 
    }

    inputRef:any;
    firstUpdated(_changed:any) {
        this.inputRef = this.shadowRoot?.getElementById("input");
        this.requestUpdate();
    }
    getSize() {
        if(!this.inputRef) {
            return 24;
        } else {
            //this isn't perfect, adds too much margin, but w/e
            return (this.inputRef.value.length * 16) + 8;
        }
    }

    @property({type: Number})
    total:number = 0;

    @property({type: Number})
    page:number = 0;

    render() {
        const {page, total} = this;

        console.log(page, page > 1);

        const prevClasses = classMap({
            ["icon-btn"]: true,
            prev: true,
            active: page > 1
        })
        const nextClasses = classMap({
            ["icon-btn"]: true,
            next: true,
            active: page < total
        })


        return html`
            <div class="${prevClasses}" @click=${this.onPrev}>
              <img-ui path="core/_common/chevron-left-grey.svg" alt="" class="left-arrow"></img-ui>
              <div>${STR_BACK}</div>
            </div>
            <div class="middle">
                ${STR_PAGE}
                <input id="input" @change=${this.onChange} @input=${() => this.requestUpdate()} value="${page}" type="number" style="width: ${this.getSize()}px" ></input>
                ${STR_OF}<span class="total">${total}</span>

                
            </div>
            <div class="${nextClasses}" @click=${this.onNext}>
              <div>${STR_NEXT}</div>
              <img-ui path="core/_common/chevron-right-grey.svg" alt="" class="left-arrow"></img-ui>
          </div>
        `
    }
}

/*
@customElement('pagination-widget')
export class _ extends LitElement {
  static get styles() {
    return [css`
   :host {
     display:flex;
   }
   .wrapper{
     display:flex;
   }
   .inner-wrapper{display:flex;}
   .left-arrow{
     transform:rotate(180deg);
     display:flex;
     margin-top:-2px;
     
   }
   p{
     margin:0;
   }
   .page-number{
    width: 24px;
    height: 24px;
    border: solid 1px #c4dbfe;
    background-color: #e6f0ff;
    display:flex;
    align-items:center;
    text-align:center;
    color: #5590fc;
    margin:0 8px;

   }
   input:focus{
     outline:none;
   }
   input[type=number]::-webkit-inner-spin-button, 
input[type=number]::-webkit-outer-spin-button { 
  -webkit-appearance: none; 
  margin: 0; 
}
.arrow{
  display:flex;
  margin-top:-6px;
}
.back{
  margin-right:55px;
  cursor:pointer;
}
.next{
  margin-left:55px;
  cursor:pointer;
}
span{
  margin-left: 4px;
}

 
    `];
  }

  @property()
  number:string =  ""

  render() {
    const {number} = this;
    const STR_BACK = "Back";
    const STR_PAGE = "Page";
    const STR_OF = "of";
    const STR_NEXT ="Next";
    return html`
    
    <main>
      <div class="wrapper">
        <div class="inner-wrapper back">
          <img-ui path="core/_common/chevron-right-grey.svg" alt="" class="left-arrow"></img-ui>
          <p>
            ${STR_BACK}
          </p>
        </div>
        <div class="inner-wrapper">${STR_PAGE}
          <input value="1" type="number"
            class="page-number">
    
          ${STR_OF}&nbsp;
          <slot></slot>
        </div>
        <div class="inner-wrapper next">
          <p>${STR_NEXT}</p>
          <img-ui path="core/_common/chevron-right-grey.svg" alt="" class="arrow"></img-ui>
        </div>
      </div>
    </main>
  `;
  }
}
*/
