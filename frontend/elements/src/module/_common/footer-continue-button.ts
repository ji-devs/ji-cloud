import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_CONTINUE = "Continue";

@customElement('module-footer-continue-button')
export class _ extends LitElement {
  static get styles() {
      return [css`
    `];
  }

  @property({type: Boolean})
  enabled:boolean = false;

  render() {
      const {enabled} = this;

      const color = enabled ? "red" : "grey";
      const pointer = enabled ? "initial" : "none";

      return html`
          <button-rect 
              .color=${color} 
              style="pointer-events: ${pointer}"
              size="small"
              iconAfter="arrow"
              @click=${() => {
                    if(enabled) {
                        this.dispatchEvent(new Event("next"));
                    }
              }}
              >${STR_CONTINUE}
              </button-rect>
      `
  }
}


        /*
        //TODO - simplify with enabled/disabled button element
        //should be able to drive it all via a simple property
        let is_ready = Rc::new(RefCell::new(false));

        html!("module-footer", {
            .future(state.step_ready_signal().for_each(clone!(is_ready => move |ready| {
                *is_ready.borrow_mut() = ready;
                async {}
            })))
            .property("slot", "footer")
            .child(html!("button-rect", {
                .style_signal("pointer-events", state.step_ready_signal().map(|ready| {
                    if ready {
                        "initial"
                    } else {
                        "none"
                    }
                }))
                .property_signal("color", state.step_ready_signal().map(|ready| {
                    if ready {
                        "red"
                    } else {
                        "grey"
                    }
                }))
                .property("size", "small")
                .property("iconAfter", "arrow")
                .property("slot", "btn")
                .text(crate::strings::STR_CONTINUE)
                .event(clone!(state, is_ready => move |evt:events::Click| {
                    if *is_ready.borrow() {
                        state.next_step();
                    }
                }))
            }))
                
        })
        */
