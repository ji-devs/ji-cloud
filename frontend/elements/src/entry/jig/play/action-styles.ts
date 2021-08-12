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
    .action.huge {
        height: 108px;
        width: 108px;
        padding: 30px;
    }
    .action.large {
        height: 62px;
        width: 62px;
        padding: 14px;
    }
    .action.small {
        height: 48px;
        width: 48px;
        padding: 10px;
    }
    .action:hover, .action:active {
        background-color: var(--dark-blue-5);
    }
    .action img-ui {
        height: 100%;
        width: 100%;
    }
`;
