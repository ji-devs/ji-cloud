import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('card-back')
export class _ extends LitElement {
    static get styles() {
        return [css`
    main{
        width:354px;
        height:384px;
        border-radius:20px;
        background-color:#2565d5;
        display:flex;
        padding-top:24px;
        flex-direction:column;
        padding-right:4px;
        z-index:2;
    }
    .main-wrapper{
        position:relative;
    
    }
        
    ::slotted([slot="subtitle"]){
        height: 24px;
       
    }
    ::slotted([slot="age"]) {
        margin-left:8px;
        height:40px;
       
    }
    ::slotted([slot="language"]) {
        margin-right:8px;
        height:40px;
       
    }
    ::slotted([slot="dropdowns"]) {
        display:flex;
        align-items:center;
        width:100%;
        position:relative;
        padding-top:16px;
        padding-bottom:8px;
       
    }
    ::slotted([slot="dropdowns"]:first-of-type) {
        padding-top:0;
       
    }
 
    .dropdown-wrapper{
        display:flex;
        align-items:center;
        flex-direction:column;
        
    }
    .wrapper{
        height:302px;
        overflow-y:auto;
        overflow-x:hidden;
        padding-left:16px;
        padding-right: 8px
        
    }
    .footer{
        display:flex;
        justify-content:center;
        align-items:center;
        height:85px;
    }

    .wrapper::-webkit-scrollbar {
        width: 4px;
    }
     
    .wrapper::-webkit-scrollbar-track {
        -webkit-box-shadow: inset 0 0 6px rgba(0,0,0,0.3);
    }
     
    .wrapper::-webkit-scrollbar-thumb {
      background-color: #89b3ff;
      outline: 1px solid #89b3ff;
      border-radius:4px;
    }
    .jig-link img-ui{
        transform:rotate(270deg)
    }
    .jig-link{
        display:flex;
    }
    .user-wrapper{
        display:flex;
        justify-content:space-between;
        align-items:center;
    }
    .jigglingpopup{
        position:absolute;
        top:-60px;
        right:0;
        z-index:-1;
    }
    .team{
        margin-top: 20px;
        display:flex;
    }
    ::slotted([slot="user"]){
        display:block;
        margin-right:4px;
    }
    
    `];
    }


    render() {

        const { } = this;

        return html` 
        <div class="main-wrapper">
            <img-ui path="group-11868.svg" class="jigglingpopup"></img-ui>
        
            <main>
                <div class="wrapper">
                    <slot name="title"></slot>
        
                    <slot name="subtitle"></slot>
        
                    <div class="dropdown-wrapper">
                        <slot name="dropdowns"></slot>
                    </div>
                    <div class="user-wrapper">
                        <div class="team">
                            <slot name="user"></slot>
                            <slot name="username"></slot>

                        </div>
                        <div class="jig-link">
                            <slot name="jigs"></slot>
                            <img-ui path="Icn_arrow_nm.svg"></img-ui>
                        </div>
                    </div>
                </div>
                <div class="footer">
                    <button-rect-icon color="blue" kind="text" iconBefore="blueplay">Play</button-rect-icon>
                </div>
            </main>
        </div>
  `;
    }
}