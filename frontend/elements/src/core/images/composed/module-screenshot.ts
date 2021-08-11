import { LitElement, html, css, customElement, property } from "lit-element";
import { imageLib, MediaLibOptions, MediaSizeOptions } from "@utils/path";
import {sameOrigin} from "@utils/image";
import {nothing} from "lit-html";
import { ModuleKind } from "@elements/module/_common/types";

@customElement("img-module-screenshot")
export class _ extends LitElement {
    @property()
    jigId: string = "";

    @property()
    moduleId: string = "";

    @property()
    size: MediaSizeOptions = "thumb";

    @property()
    fallbackKind: ModuleKind = "cover";

    render() {
	    const {jigId, moduleId, fallbackKind, size} = this;

	    return html`
	    	<img-ji lib="screenshot" id="${jigId}/${moduleId}" size="${size}">
  			<img-ui path="module/${fallbackKind}/thumb-placeholder.jpg" slot="fallback"></img-ui>
		</img-ji>
	    `;
    }
}
