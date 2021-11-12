import { css } from "lit-element";

export const collapseStyles = css`
    :host {
        --fading-phase-duration: 0.2s;
        --collapsing-phase-duration: 0.3s;
    }
    @keyframes hide {
        from {
        }
        to {
            height: 0;
            width: 0;
            overflow: hidden;
        }
    }
    @keyframes show {
        from {
            height: 0;
            width: 0;
            overflow: hidden;
        }
        to {
        }
    }
    .open-only {
        transition-property: opacity;
        transition-delay: var(--collapsing-phase-duration);
        transition-duration: var(--fading-phase-duration);
        transition-timing-function: linear;

        animation-name: show;
        animation-fill-mode: forwards;
        animation-delay: 0s;
        animation-duration: 0s;
        animation-timing-function: linear;

        opacity: 1;
    }
    :host([collapsed]) .open-only {
        transition-delay: 0s;
        animation-name: hide;
        animation-delay: var(--fading-phase-duration);
        opacity: 0;
    }
    .collapsing-phase {
        transition-timing-function: linear;
        transition-delay: 0s;
        transition-duration: var(--collapsing-phase-duration);
    }
    :host([collapsed]) .collapsing-phase {
        transition-delay: var(--fading-phase-duration);
    }
`;
