/// <reference types="google.maps" />
import { LitElement, html, css, customElement, property } from "lit-element";

const Key = {
    ArrowDown: "ArrowDown",
    ArrowUp: "ArrowUp",
    Enter: "Enter",
    Escape: "Escape",
} as const;

@customElement("input-location")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: relative;
                    display: inline-block;
                }
                input {
                    all: inherit;
                    width: 100%;
                    height: 100%;
                }
                input::placeholder {
                    color: var(--light-gray-4);
                }
                .dropdown {
                    position: absolute;
                    top: 100%;
                    left: 0;
                    right: 0;
                    z-index: 1000;
                    background: white;
                    border: 1px solid #ccc;
                    border-top: none;
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                    list-style: none;
                    margin: 0;
                    padding: 0;
                    max-height: 300px;
                    overflow-y: auto;
                }
                .dropdown li {
                    padding: 8px 12px;
                    cursor: pointer;
                    font-size: 14px;
                    color: #333;
                }
                .dropdown li.active {
                    background: #e8e8e8;
                }
                .dropdown li:hover {
                    background: #f0f0f0;
                }
                .dropdown li.loading {
                    color: #999;
                    cursor: default;
                }
            `,
        ];
    }

    @property()
    value: string = "";

    @property()
    placeholder: string = "";

    set locationAsString(v: string) {
        if (v) {
            this.value = JSON.parse(v)?.input;
        } else {
            this.value = "";
        }
    }

    private predictions: Array<{ description: string; placeId: string }> = [];
    private activeIndex: number = -1;
    private loading: boolean = false;

    focus() {
        this.shadowRoot?.getElementById("input")?.focus();
    }

    onLocation(data: { input: string; place: any } | null) {
        let detail = {};
        if (data != null) {
            detail = {
                ...data,
                rawJson: JSON.stringify(data),
            };
        }
        this.dispatchEvent(new CustomEvent("google-location", { detail }));
    }

    private async fetchPredictions(input: string) {
        if (!input.trim()) {
            this.predictions = [];
            this.activeIndex = -1;
            this.requestUpdate();
            return;
        }

        if (this.predictions.length === 0) {
            this.loading = true;
            this.requestUpdate();
        }

        try {
            const request = {
                input,
                includedPrimaryTypes: ["geocode"],
            };
            const { suggestions } = await google.maps.places.AutocompleteSuggestion.fetchAutocompleteSuggestions(request);

            this.predictions = suggestions
                .filter((s) => s.placePrediction)
                .filter((s) => {
                    // Filter out country-only results (results without secondary text are typically countries)
                    return !!s.placePrediction?.secondaryText;
                })
                .map((s) => ({
                    description: s.placePrediction!.text.text,
                    placeId: s.placePrediction!.placeId,
                }));
        } catch (_e) {
            this.predictions = [];
        }
        this.activeIndex = -1;
        this.loading = false;
        this.requestUpdate();
    }

    private async selectPrediction(prediction: { description: string; placeId: string }) {
        const inputEl = this.shadowRoot?.getElementById("input") as HTMLInputElement;
        const description = prediction.description;

        this.value = description;
        if (inputEl) {
            inputEl.value = description;
        }
        this.predictions = [];
        this.activeIndex = -1;
        this.requestUpdate();

        try {
            const place = new google.maps.places.Place({ id: prediction.placeId });
            await place.fetchFields({ fields: ["addressComponents"] });

            // Convert new API format to legacy format
            const legacyPlace = {
                address_components: (place.addressComponents || []).map((c: any) => ({
                    long_name: c.longText || "",
                    short_name: c.shortText || "",
                    types: c.types || [],
                })),
            };

            this.onLocation({ input: description, place: legacyPlace });
        } catch (e) {
            this.onLocation(null);
        }
    }

    private onInput(evt: InputEvent) {
        const { value } = evt.target as HTMLInputElement;
        this.value = value;

        this.dispatchEvent(
            new CustomEvent("custom-input", {
                detail: { value },
            })
        );

        this.fetchPredictions(value);
    }

    private onKeyDown(evt: KeyboardEvent) {
        if (!this.predictions.length) {
            if (evt.key === Key.ArrowDown && this.value.trim()) {
                evt.preventDefault();
                this.fetchPredictions(this.value);
            }
            return;
        }

        if (evt.key === Key.ArrowDown) {
            evt.preventDefault();
            this.activeIndex = Math.min(this.activeIndex + 1, this.predictions.length - 1);
            this.requestUpdate();
        } else if (evt.key === Key.ArrowUp) {
            evt.preventDefault();
            this.activeIndex = Math.max(this.activeIndex - 1, 0);
            this.requestUpdate();
        } else if (evt.key === Key.Enter) {
            evt.preventDefault();
            if (this.activeIndex >= 0 && this.activeIndex < this.predictions.length) {
                this.selectPrediction(this.predictions[this.activeIndex]);
            }
        } else if (evt.key === Key.Escape) {
            this.predictions = [];
            this.activeIndex = -1;
            this.requestUpdate();
        }
    }

    private onBlur() {
        // Delay to allow click on dropdown item to fire first
        setTimeout(() => {
            this.predictions = [];
            this.activeIndex = -1;
            this.requestUpdate();
        }, 200);
    }

    private renderDropdown() {
        if (this.loading) {
            return html`
                <ul class="dropdown">
                    <li class="loading">Loading...</li>
                </ul>
            `;
        }

        if (this.predictions.length === 0) {
            return html``;
        }

        return html`
            <ul class="dropdown">
                ${this.predictions.map((p, i) => this.renderPrediction(p, i))}
            </ul>
        `;
    }

    private renderPrediction(prediction: { description: string; placeId: string }, index: number) {
        const classes = index === this.activeIndex ? "active" : "";
        return html`
            <li
                class="${classes}"
                @mousedown="${() => this.selectPrediction(prediction)}"
            >
                ${prediction.description}
            </li>
        `;
    }

    render() {
        return html`
            <input
                id="input"
                placeholder="${this.placeholder}"
                type="text"
                .value="${this.value}"
                @input="${this.onInput}"
                @keydown="${this.onKeyDown}"
                @blur="${this.onBlur}"
                autocomplete="off"
            />
            ${this.renderDropdown()}
        `;
    }
}
