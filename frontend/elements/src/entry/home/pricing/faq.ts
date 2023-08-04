import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("pricing-faq")
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                max-width: 664px;
                margin: 0 auto;
                padding-bottom: 24px;
                display: block;
            }
            h2 {
                margin: 44px 0;
                text-align: center;
                color: var(--dark-blue-4);
                font-size: 28px;
                font-weight: 900;
            }
            details {
                border-bottom: var(--light-gray-2) 1px solid;
            }
            summary {
                font-size: 18px;
                color: var(--dark-gray-6);
                height: 64px;
                display: grid;
                grid-template-columns: 64px 1fr;
                align-items: center;
                cursor: pointer;
            }
            /* This hides the caret in Safari */
            summary::-webkit-details-marker {
                display: none;
            }
            details summary fa-icon {
                text-align: center;
                rotate: 0deg;
                transition: rotate .2s;
            }
            details[open] summary fa-icon {
                rotate: 90deg;
            }
            details p {
                grid-column: 2;
                margin: 0;
                margin-left: 64px;
                margin-bottom: 20px;
                font-size: 14px;
                line-height: 1.4;
                color: var(--dark-gray-6);
            }
        `];
    }

    render() {
        return html`
            <h2>FAQs</h2>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    What happens when the trial for a subscription expires?
                </summary>
                <p>When the subscription’s trial period expires, your paid subscription will begin immediately. You will get a reminder by email 24 hours before your free trial is due to expire and you can cancel at any time. To do this, go to My subscription in the top-right account menu then click Change Plan.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    When will payments be taken?
                </summary>
                <p>When the subscription’s trial period expires, your paid subscription will begin immediately. You will get a reminder by email 24 hours before your free trial is due to expire and you can cancel at any time. To do this, go to My subscription in the top-right account menu then click Change Plan.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    Can I switch subscriptions later on?
                </summary>
                <p>When the subscription’s trial period expires, your paid subscription will begin immediately. You will get a reminder by email 24 hours before your free trial is due to expire and you can cancel at any time. To do this, go to My subscription in the top-right account menu then click Change Plan.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    Is there a contract? or can I cancel anytime?
                </summary>
                <p>When the subscription’s trial period expires, your paid subscription will begin immediately. You will get a reminder by email 24 hours before your free trial is due to expire and you can cancel at any time. To do this, go to My subscription in the top-right account menu then click Change Plan.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    Can I get an invoice?
                </summary>
                <p>When the subscription’s trial period expires, your paid subscription will begin immediately. You will get a reminder by email 24 hours before your free trial is due to expire and you can cancel at any time. To do this, go to My subscription in the top-right account menu then click Change Plan.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    How can I change my credit card or payment method?
                </summary>
                <p>When the subscription’s trial period expires, your paid subscription will begin immediately. You will get a reminder by email 24 hours before your free trial is due to expire and you can cancel at any time. To do this, go to My subscription in the top-right account menu then click Change Plan.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    Can I switch between monthly and annual subscriptions?
                </summary>
                <p>When the subscription’s trial period expires, your paid subscription will begin immediately. You will get a reminder by email 24 hours before your free trial is due to expire and you can cancel at any time. To do this, go to My subscription in the top-right account menu then click Change Plan.</p>
            </details>
        `;
    }
}
