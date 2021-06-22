import { css } from "lit-element";

export default css`
    :host {
        position: relative;
        padding: 32px;
        display: block;
        border-radius: 16px;
        box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
        max-width: 700px;
        background-color: #ffffff;
    }
    ::slotted([slot=close]) {
        height: 32px;
        width: 32px;
        position: absolute;
        top: 7px;
        right: 7px;
        display: inline-grid;
        place-content: center;
        font-size: 30px;
        font-weight: 200;
    }
    .divider {
        background-color: #d5e4ff;
        height: 1px;
        margin: 20px 0;
    }
    h2 {
        font-size: 32px;
        font-weight: bold;
        color: var(--orange);
        margin: 0;
    }
    .actions {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
`;
