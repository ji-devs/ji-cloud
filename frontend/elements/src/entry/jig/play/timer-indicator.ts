import { customElement } from "lit-element";
import { IndicatorBase } from "./indicator-base";

@customElement("jig-play-timer-indicator")
export class _ extends IndicatorBase {
    render() {
        return this.renderIndicator("timer");
    }
}
