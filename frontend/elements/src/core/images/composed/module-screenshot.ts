import { LitElement, html, css, customElement, property } from "lit-element";
import { imageLib, MediaLibOptions, MediaSizeOptions } from "@utils/path";
import {sameOrigin} from "@utils/image";
import {nothing} from "lit-html";
import { ModuleKind } from "@elements/module/_common/types";

@customElement("img-module-screenshot")
export class _ extends LitElement {
    static get styles() {
        return [css`
            img-ui, img-ji {
                height: 100%;
                width: 100%;
            }
        `]
    }

    @property()
    jigId: string = "";

    @property()
    moduleId: string = "";

    @property()
    size: MediaSizeOptions = "thumb";

    @property()
    moduleKind: ModuleKind | "" = "";

    render() {
	    const {jigId, moduleId, moduleKind, size} = this;

        const fallbackPath = moduleKind == ""
            ? `jig/thumb-placeholder.svg` 
            : `module/${moduleKind}/thumb-placeholder.svg`;
	    return html`
	    	<img-ji lib="screenshot" id="${jigId}/${moduleId}" size="${size}">
  			<img-ui path="${fallbackPath}" slot="fallback"></img-ui>
		</img-ji>
	    `;
    }
}
