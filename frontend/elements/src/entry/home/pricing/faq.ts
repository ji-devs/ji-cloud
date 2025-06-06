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
                min-height: 64px;
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
                    What happens when the free trial for a subscription expires?
                </summary>
                <p>When the subscription’s trial period expires, your paid subscription will begin immediately. You will get a reminder by email 24 hours before your free trial is due to expire (Individual monthly & annual subscriptions). If you wish to cancel at any time during your trial period, go to <strong>Account</strong> in the top-right menu then, scroll down to <strong>My current plan</strong>, click on the pink <strong>Manage my plan</strong> button and then you will have an option to <strong>Cancel plan</strong> which will cancel your plan at the end of the free trial and you will not be charged.</p>
                <p>For School subscriptions, if you didn’t enter payment details at the start of your free trial, you will be contacted towards the end of the 14-day period with payment options in order to confirm the (annual) subscription.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    Can I cancel my paid subscription anytime?
                </summary>
                <p>After the free trial period has ended, you can downgrade to our free plan at any time and no further payments will be taken. To do this go to <strong>Account</strong> in the top-right menu, scroll down to <strong>My current plan</strong>, and click on the pink <strong>Manage my plan</strong> button and then you will have an option to <strong>Cancel plan</strong>. You will still have full access to your subscription until the end of the billing period.</p>
                <p>Instead of canceling, you can click the <strong>Pause subscription</strong> button if you have a monthly plan. This will pause all payments until you click the <strong>Unpause subscription</strong> button. While your plan is paused, you will not have access to premium content or tools.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    When will payments be taken?
                </summary>
                <p>The first payment is taken at the end of your free trial period and then recurring payments on the same day each month thereafter (monthly subscriptions). You will get a reminder by email 24 hours before your monthly renewal. For annual subscriptions you will be charged once for the year after your trial period has finished. You will get a reminder 30 days before your annual renewal.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    How can I change my payment card details?
                </summary>
                <p>If a payment has already been made, we cannot refund and recharge this payment. However, if you would like to change payment methods for any upcoming charges, go to <strong>Account</strong> in the top-right account menu, scroll down to <strong>My current plan</strong> and then click the pink <strong>Manage my plan</strong> button. There you can change your payment method.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    Can I switch subscriptions later on?
                </summary>
                <p>Yes, you can upgrade your plan from a monthly to an annual plan. You can also upgrade from Basic to Pro in an individual account or increase the number of teachers in a school account. But, to downgrade, you need to contact us by emailing info@jigzi.org.</p>
                <p>To make changes to your plan, go to <strong>Account</strong> in the top-right account menu, scroll down to <strong>My current plan</strong> and select one of the options available there.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    If I cancel, can I resume a subscription later?
                </summary>
                <p>Yes, your JIGs, playlists, & resources will remain in our cloud system and you’ll still be able to use all of the features included in your FREE account. When you decide to return and renew a paid subscription, you will regain full access to premium content and tools provided in the paid plan.</p>
                <p>We recommend that instead of canceling, you click the <strong>Pause subscription</strong> if you have a monthly subscription. Then you can <strong>Unpause subscription</strong> later to resume.</p>
            </details>
            <details>
                <summary>
                    <fa-icon icon="fa-regular fa-angle-right"></fa-icon>
                    What happens to my JIGs, playlists, resources, and courses if I downgrade to a FREE account?
                </summary>
                <p>Anything you have created and published will continue to work as before. You will still be able to edit, play, and share your creations, however, you will no longer be able to create playlists, resources, or courses, and if you have already created 5 JIGs you will not be able to create new ones.</p>
            </details>
        `;
    }
}
