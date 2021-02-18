import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/core/inputs/text-pencil";
import "@elements/core/buttons/icon";
import "@elements/core/buttons/text";
import {ModuleKind, GET_STR_MODULE} from "@elements/entry/jig/module-types";

@customElement("jig-edit-sidebar-module")
export class _ extends LitElement {
  static get styles() {
    return [
        css`
            section {
              width: 416px;
              height: 168px;
                display: flex;
            }
            .grid-container {
                margin-top: 23px;
                display: grid;
              grid-template-columns: 126px 218px 1fr;
              grid-template-rows: 1fr;
              gap: 0px 0px;
              grid-template-areas:
                "left middle right";
            }

            .left {
                padding-left: 16px;
                grid-area: left;
                display: flex;
                flex-direction: column;
            }

            .left.selected {
            }
            .middle {
                grid-area: middle;
            }
            .right {
                grid-area: right;
                margin-left: 16px;
                display: flex;
                z-index: 1;
            }
            section.selected {
              border: solid 1px #e7f0fd;
              background-color: #f8f9fd;
            }

            aside.selected {
                background-color: var(--Main_Blue);
            }
            aside {
              width: 8px;
              height: 100%;
            }
            .title {
              font-size: 20px;
              font-weight: bold;
              font-stretch: normal;
              font-style: normal;
              line-height: 1.5;
              letter-spacing: 0.6px;
              text-align: left;
              color: var(--Main_Blue);
            }
            .subtitle {
              font-size: 16px;
              font-weight: 500;
              font-stretch: normal;
              font-style: normal;
              line-height: 1.5;
              letter-spacing: normal;
              text-align: left;
              color: #4a4a4a;
            }
            .icon {
                margin-top: 8px;
            }
            .window {
                position: relative;
                z-index: 1;
            }
            .decorations {
                position: relative;
                top: 0;
                left: 0;
            }

            .add-container {
                position: relative;
                top: 0px; 
                left: 0px; 
                z-index: 1;
            }
            .add {
                position: absolute;
                top: -15px; 
                left: calc(416px - (30px + 17px)); 
            }
            .arm-left, .arm-right, .neck, .head, .torso-columns, .torso-gears, .torso-spring, .feet-spring, .feet-rollers {
                position: absolute;
                top: 0;
                left: 0;
            }

            .arm-left {
                transform: translate(-35px, -10px);
            }
            .arm-right {
                transform: translate(190px, -110px);
            }
            .neck {
                transform: translate(92px, -60px); 
            }
            .head {
                transform: translate(40px, -200px); 
            }
            .torso-columns {
                transform: translate(61px, 110px); 
            }
            .torso-gears {
                transform: translate(60px, 120px); 
            }
            .torso-spring {
                transform: translate(86px, 110px); 
            }
            .feet-spring {
                transform: translate(92px, 90px); 
            }
            .feet-rollers{
                transform: translate(49px, 150px); 
            }
            `,
    ];
  }

  @property({type: Boolean})
  selected:boolean = false;

  //Should be the raw index in the JIG's module list
  //Will be bumped by 1 for display purposes
  @property({type: Number})
  index:number = 0;

  @property({type: Boolean})
  lastBottomDecoration:boolean = false;

  @property()
  module:ModuleKind | "" = "";

  render() {
      const {selected, index, lastBottomDecoration, module} = this;
      const sectionClasses = classMap({selected});
      const asideClasses = classMap({selected});

      const title = (index+1).toString().padStart(2, '0');

      const subtitle = module === "" ? "" 
          : GET_STR_MODULE(module);

      const iconPath = module === "" ? "" 
          : `entry/jig/modules/small/${module}.svg`;

      return html`
          <section class="${sectionClasses}">
              <aside class="${asideClasses}"></aside>
              <div class="grid-container">
                  <div class="left">
                      <div class="title">${title}</div>
                      ${subtitle === "" ? nothing
                          : html`<div class="subtitle">${subtitle}</div>`
                      }
                      ${iconPath === "" ? nothing
                          : html`<img-ui class="icon" path="${iconPath}"></img-ui>`
                      }
                  </div>
                  <div class="middle">
                      <div class="decorations">
                          ${renderDecoration(module, index, lastBottomDecoration)}
                      </div>
                      <div class="window">
                          <slot name="window"></slot>
                        </div>
                  </div>
                  <div class="right">
                      <slot name="menu"></slot>
                  </div>
          </section>
          <div class="add-container">
              <div class="add">
                  <slot name="add"></slot>
              </div>
          </div>
      `;
  }
}

function renderDecoration(module: ModuleKind | "", index: number, lastBottomDecoration: boolean) {
    const getImage = (path:string, classes:string) => html`<img-ui class="${classes}" path="entry/jig/jiggling/${path}" />`;
    
    const renderBottomDecoration = () => {
        return html`
            ${getImage("feet-spring.svg", "feet-spring")}
            ${getImage("yellow/feet-rollers.svg", "feet-rollers")}
        `
    }
    if(module === "cover") {
        return html`
            ${getImage("arm-left.svg", "arm-left")}
            ${getImage("arm-right.svg", "arm-right")}
            ${getImage("neck-spring.svg", "neck")}
            ${getImage("yellow/face.png", "head")}
            ${lastBottomDecoration 
                ? renderBottomDecoration() 
                : getImage("torso-columns.svg", "torso-columns")
            }
        `
    } else if(lastBottomDecoration) {
        return renderBottomDecoration();
    } else {
        switch(index % 3) {
            case 0: return getImage("torso-columns.svg", "torso-columns");
            case 1: return getImage("torso-spring.svg", "torso-spring");
            case 2: return getImage("torso-gears.svg", "torso-gears");
            default: return nothing;
        }
    }
}
