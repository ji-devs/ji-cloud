import { LitElement, html, css, customElement, property } from 'lit-element';
import { createPopper, Placement, VirtualElement, Instance as PopperInstance} from '@popperjs/core';
export { Placement} from '@popperjs/core';
export type COLOR = "beige" | "red";

@customElement("tooltip-base")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    box-shadow: 0 3px 40px 0 rgba(0, 0, 0, 0.08);
                    --arrow-size: 24px;
                    --arrow-offset: -10px;
                    z-index: 1000; /* is still restricted by shadow dom? */
                }

                :host([color="beige"]) {
                    border: solid 2px var(--light-orange-2);
                    background-color: var(--light-orange-1);
                }
                :host([color="red"]) {
                    background-color: var(--light-red-1);
                }
                
                .content {
                    padding: 24px;
                }
                #arrow,
                #arrow::before {
                  position: absolute;
                  width: var(--arrow-size);
                  height: var(--arrow-size);
                  background: inherit;
                }

                #arrow {
                  visibility: hidden;
                }

                #arrow::before {
                  visibility: visible;
                  content: '';
                  transform: rotate(45deg);
                }

                :host([data-popper-placement^='top'])  #arrow {
                  bottom: var(--arrow-offset);
                }

                :host([data-popper-placement^='bottom'])  #arrow {
                  top: var(--arrow-offset);
                }

                :host([data-popper-placement^='left'])  #arrow {
                  right: var(--arrow-offset);
                }

                :host([data-popper-placement^='right'])  #arrow {
                  right: var(--arrow-offset);
                }
            `
        ];
    }

    /*
    createRenderRoot() {
      return this;
      }
     */
    popperInstance:PopperInstance | undefined;

    firstUpdated(_changed:any) {
        this.bindPopper();
        //TODO - instantiate popper...
    }

    updated(changed:any) {
        if(typeof changed.get("target") === "boolean") {
            this.bindPopper();
        }
    }

    bindPopper = () => {
        if(this.target) {
            this.killPopper();

            this.popperInstance = createPopper(this.target, this, {
                placement: this.placement,
                modifiers: [
                    {
                      name: "arrow",
                      options: {
                          element: this.shadowRoot?.getElementById("arrow"),
                          padding: 12,
                      }
                    },
                    {
                      name: "offset",
                      options: {
                          offset: [this.offsetSkidding, this.offsetDistance],
                      }
                    }
                  ]
            });
        }
    }

    killPopper = () => {
        if(this.popperInstance != undefined) {
            this.popperInstance.destroy();
        }
        this.popperInstance = undefined;

    }

    disconnectedCallback() {
        this.killPopper();
    }

    @property({type: Number})
    offsetSkidding:number = 0;

    @property({type: Number})
    offsetDistance:number = 0;

    @property({reflect: true})
    color:COLOR = "beige";

    @property()
    target:Element | VirtualElement | undefined;

    @property()
    placement:Placement = "left";

    render() {
        return html`
            <div class="content"><slot></slot></div>
            <div id="arrow" data-popper-arrow></div>

        `;
    }
}
