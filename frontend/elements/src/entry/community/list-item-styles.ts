import { css } from "lit-element";

export const listItemStyles = css`
    :host {
        padding-left: 20px;
        padding-right: 20px;
        display: grid;
        align-items: center;
        justify-content: start;
        column-gap: 20px;
    }
    :host(community-list-circle-header), :host(community-list-circle) {
        grid-template-columns: 84px 160px;
    }
    :host(community-list-member-header), :host(community-list-member) {
        grid-template-columns: 50px 160px;
    }
    .desktop-only {
        display: none;
    }
    @media (min-width: 1024px) {
        :host {
            justify-content: space-between;
        }
        :host(community-list-circle-header), :host(community-list-circle) {
            grid-template-columns: 84px 160px 40px 300px 108px;
        }
        :host(community-list-member-header), :host(community-list-member) {
            grid-template-columns: 50px 160px 80px 80px 108px;
        }
        .desktop-only {
            display: inline-block;
        }
    }
`;

export const circleTemplatesColumns = "108px 200px 50px 400px 138px";
export const memberTemplatesColumns = "64px 200px 100px 100px 138px";
