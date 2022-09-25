import { css } from "lit-element";

export const actionStyles = css`
    .action {
        cursor: pointer;
        display: inline-flex;
        place-content: center;
        box-sizing: border-box;
        border: solid var(--light-blue-3) 1px;
        border-radius: 50%;
        background-color: var(--dark-blue-8);
        box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
    }
    .action.large {
        height: 62px;
        width: 62px;
        padding: 14px;
        font-size: 34px;
    }
    .action.small {
        height: 48px;
        width: 48px;
        padding: 10px;
        font-size: 28px;
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

    /* mobile */
    @media (max-width: 1000px) {
        .action.large {
            height: 32px;
            width: 32px;
            padding: 6px;
            font-size: 20px;
        }
        .action.small {
            height: 24px;
            width: 24px;
            padding: 5px;
            font-weight: 14px;
        }
    }
`;
