import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

const STR_LABEL = `Location*`;

@customElement("input-location-old")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .input-wrapper {
                    position: relative;
                    width: inherit;
                    height: 64px;
                    border: solid 1px #89b3ff;
                    border-radius: 14px;
                    padding: 8px 48px 8px 16px;
                }
                .errorwrapper {
                    border: solid 1px #f00813;
                    background-color: #fff4f4;
                }
                .errorwrapper input {
                    background-color: #fff4f4;
                }

                input {
                    outline: none;
                    border: none;
                    margin-top: 33px;
                    width: inherit;
                }
                label {
                    position: absolute;
                    top: 0;
                    left: 0;
                    font-size: 16px;
                    padding: 8px 0px 0px 16px;
                    color: #5590fc;
                }
                .input-wrapper:active {
                    border: solid 2px #5590fc;
                    margin: -1px;
                }
                .input-wrapper:focus {
                    border: solid 2px #5590fc;
                    margin: -1px;
                }
                input {
                    font-size: 16px;
                    width: 100%;
                }

                .error {
                    font-size: 14px;
                    color: #f00813;
                    margin-top: 4px;
                    font-weight: 500;
                    padding-left: 8px;
                    display: block;
                }
                .instruction {
                    font-size: 14px;
                    color: #4a4a4a;
                    margin-top: 4px;
                    font-weight: 500;
                    padding-left: 8px;
                }

                img-ui {
                    position: absolute;
                    top: 33%;
                    right: 12px;
                    cursor: pointer;
                }
            `,
        ];
    }

    @property()
    value: string = "";

    // will also change the error wrapper internally
    @property()
    error: string = "";

    @property()
    help: string = "";

    @property()
    placeholder: string = "";

    private autoComplete: any;

    onLocation(data: { input: string; place: any } | null) {
        this.dispatchEvent(
            new CustomEvent("google-location", {
                detail:
                    data == null
                        ? {}
                        : {
                              ...data,
                              rawJson: JSON.stringify(data),
                          },
            })
        );
    }

    firstUpdated() {
        const input = this.shadowRoot?.getElementById(
            "input"
        ) as HTMLInputElement;
        this.autoComplete = bindGoogleMaps(input, this.onLocation.bind(this));
    }

    onFocus() {
        geolocate(this.autoComplete);
    }

    render() {
        const { help, placeholder, error, value } = this;

        const isError: boolean = error !== "";

        const isHelp: boolean = help !== "";

        const errorwrapper = isError ? "errorwrapper" : "";

        return html`
            <div class="input-wrapper ${errorwrapper}">
                <input
                    id="input"
                    placeholder="${placeholder}"
                    type="text"
                    value="${value}"
                    @focus="${this.onFocus}"
                />
                <label class="">${STR_LABEL}</label>
            </div>

            ${isHelp ? html`<p class="instruction">${help}</p>` : nothing}
            ${isError ? html`<p class="error">${error}</p>` : nothing}
        `;
    }
}

function bindGoogleMaps(
    elem: HTMLInputElement,
    onLocation: (data: { input: string; place: any } | null) => any
) {
    // Create the autocomplete object, restricting the search predictions to
    // geographical location types.
    const autoComplete = new google.maps.places.Autocomplete(elem, {
        types: ["geocode"],
    });
    // Avoid paying for data that you don't need by restricting the set of
    // place fields that are returned to just the address components.
    autoComplete.setFields(["address_component"]);
    // When the user selects an address from the drop-down, populate the
    // address fields in the form.
    autoComplete.addListener("place_changed", () => {
        const place = autoComplete.getPlace();
        const value = elem.value;
        onLocation(
            place == null || value == null || value == ""
                ? null
                : { input: value, place }
        );
    });

    return autoComplete;
}

// Bias the autocomplete object to the user's geographical location,
// as supplied by the browser's 'navigator.geolocation' object.
function geolocate(autoComplete: any) {
    if (navigator.geolocation) {
        navigator.geolocation.getCurrentPosition((position) => {
            const geolocation = {
                lat: position.coords.latitude,
                lng: position.coords.longitude,
            };
            const circle = new google.maps.Circle({
                center: geolocation,
                radius: position.coords.accuracy,
            });
            autoComplete.setBounds(circle.getBounds());
        });
    }
}

/*
function fillInAddress() {
  // Get the place details from the autocomplete object.

  for (const component in componentForm) {
    document.getElementById(component).value = "";
    document.getElementById(component).disabled = false;
  }

  // Get each component of the address from the place details,
  // and then fill-in the corresponding field on the form.
  for (const component of place.address_components) {
    const addressType = component.types[0];

    if (componentForm[addressType]) {
      const val = component[componentForm[addressType]];
      document.getElementById(addressType).value = val;
    }
  }
}
*/
