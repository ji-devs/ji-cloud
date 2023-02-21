import { css } from "lit-element";

export const actionStyles = css`
    .action {
        cursor: pointer;
        display: inline-flex;
        place-content: center;
        box-sizing: border-box;
        border: solid #fff 2px;
        border-radius: 50%;
        background-color: var(--dark-blue-8);
        box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
        color: #fff;
        line-height: 1em;
        height: 30px;
        width: 30px;
        font-size: 15px;
    }
    @media (min-width: 1024px) {
        .action.large {
            height: 62px;
            width: 62px;
            padding: 14px;
            font-size: 34px;
        }
        .action.middle {
            height: 38px;
            width: 38px;
            padding: 8px;
            font-size: 20px;
        }
        .action.small {
            height: 32px;
            width: 32px;
            font-size:16px;
        }
    }
    .action:hover,
    .action:active {
        background-color: var(--dark-blue-5);
    }
    .action img-ui {
        height: 100%;
        width: 100%;
    }
    .action fa-icon {
        display: grid;
    }
`;
