import {LitElement, html, css, customElement, property, PropertyValues} from "lit-element";
import {Stripe, StripeElements, loadStripe} from "@stripe/stripe-js"

@customElement("stripe-payment-details")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                }
            `,
        ];
    }

    stripe: Stripe | null = null;
    elements: StripeElements | null = null;

    @property()
    publishableKey: string = "";

    @property()
    clientSecret?: string;

    @property()
    redirectUrl: string = "";

    @property()
    buttonLabel: string = "Start trial";

    constructor() {
        super();
    }

    async onClick(e: Event) {
        e.preventDefault();
        e.stopPropagation();

        if (this.stripe && this.elements) {
            const {error} = await this.stripe.confirmSetup({
                elements: this.elements,
                confirmParams: {
                    return_url: this.redirectUrl,
                }
            });

            if (error) {
                console.log('TODO!');
            }
        }
    }

    render() {
        return html`
            <form>
                <div id="paymentMethod"></div>
                <div>
                    <button-rect @click="${this.onClick}">${this.buttonLabel}</button-rect>
                </div>
            </form>
        `;
    }

    protected createRenderRoot() {
        // Stripe elements UI uses a bunch of iframes to do its work, which does not work well with shadow root.
        // Sad times.
        return this;
    }

    async updated() {
        if (this.clientSecret && !this.stripe) {
            this.stripe = await loadStripe(this.publishableKey);
            await this.updateComplete;
            if (this.stripe) {
                this.elements = this.stripe.elements({
                    clientSecret: this.clientSecret,
                    appearance: {
                        theme: 'flat',
                    },
                });
                await this.updateComplete;

                const paymentMethodEl = this.renderRoot.querySelector("#paymentMethod") as HTMLElement;
                if (paymentMethodEl) {
                    const paymentElement = this.elements?.create("payment", {
                        layout: {
                            type: 'tabs',
                            defaultCollapsed: false
                        },
                        paymentMethodOrder: ['card', 'apple_pay', 'google_pay', 'link'],
                    });
                    paymentElement.mount(paymentMethodEl);
                }
            }
        }
    }
}
