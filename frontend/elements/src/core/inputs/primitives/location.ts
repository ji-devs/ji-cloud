import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("input-location")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                input {
                    all: inherit;
                }
            `,
        ];
    }

    @property()
    value: string = "";

    @property()
    placeholder: string = "";

    set locationAsString(v: string) {
        if(v) {
            this.value = JSON.parse(v)?.input;
        } else {
            this.value = "";
        }
    }

    private autoComplete: any;

    onLocation(data: {input: string, place: any} | null) {
        
        this.dispatchEvent(
            new CustomEvent("google-location", {
                detail: data == null
                    ? {}
                    : {
                        ...data,
                        rawJson: JSON.stringify(data),
                    }
            })
        );
    }

    firstUpdated() {
        const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
        this.autoComplete = bindGoogleMaps(input, this.onLocation.bind(this));
    }

    onFocus() {
        geolocate(this.autoComplete);
    }

    render() {
        return html`
            <input
                id="input"
                placeholder="${this.placeholder}"
                type="text"
                value="${this.value}"
                @focus="${this.onFocus}"
            />
        `;
    }
}

function bindGoogleMaps(elem: HTMLInputElement, onLocation: (data:{input: string, place: any} | null) => any) {
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
                            : { input: value, place },
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
