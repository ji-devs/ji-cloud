import { LitElement, html, css, customElement, property } from "lit-element";
import { MediaSizeOptions } from "@utils/path";
import { ModuleKind } from "@elements/module/_common/types";

@customElement("img-module-screenshot")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    box-sizing: border-box;
                    overflow: hidden;
                    display: inline-block;
                }
                img-ui,
                img-ji {
                    height: 100%;
                    width: 100%;
                }
            `,
        ];
    }

    @property()
    jigId: string = "";

    //use with cacheBust true to force reloading when id changes to the same thing
    @property({ hasChanged: () => true })
    moduleId?: string = "";

    @property()
    size: MediaSizeOptions = "thumb";

    @property()
    moduleKind: ModuleKind | "" = "";

    @property({ type: Boolean })
    cacheBust: boolean = false;

    render() {
        const { jigId, moduleId, moduleKind, size, cacheBust } = this;

        const fallbackPath =
            moduleKind == ""
                ? `jig/thumb-placeholder.svg`
                : `module/_common/thumb-placeholder.svg`;
        return html`
            ${this.moduleId ? (
                html`
                    <img-ji
                        lib="screenshot"
                        id="${jigId}/${moduleId}"
                        size="${size}"
                        .cacheBust=${cacheBust ? Date.now() : false}
                    >
                        ${this.fallbackImage(fallbackPath)}
                    </img-ji>
                `
            ) : (
                this.fallbackImage(fallbackPath)
            ) }
        `;
    }

    private fallbackImage(path: string) {
        return html`
            <img-ui path="${path}" slot="fallback"></img-ui>
        `;
    }
}
