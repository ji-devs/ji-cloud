import { css } from "lit-element";

export const scrollbarStyles = css`
    .scrollbar {
        scrollbar-width: thin;
        scrollbar-color: #d3d4dd transparent;
    }
    .scrollbar::-webkit-scrollbar-track {
        background-color: transparent;
    }
    .scrollbar::-webkit-scrollbar {
        width: 6px;
    }
    .scrollbar::-webkit-scrollbar-thumb {
        border-radius: 3px;
        background-color: #d3d4dd;
    }
    .scrollbar::-webkit-scrollbar-button {
        background-color: transparent;
    }
`;
