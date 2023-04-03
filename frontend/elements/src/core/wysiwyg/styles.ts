import { css, unsafeCSS } from "lit-element";
import { html, nothing } from "lit-html";
import { CSSProperties } from "react";
import {
    EditorElement,
    EditorText,
} from "./slate-wysiwyg-react/EditorBackbone";
import { Align, WysiwygValue } from "./wysiwyg-types";

export const baseStyles = css`
    :host {
        display: inline-block;
        padding: 0 5px;
    }
    p {
        margin: 0px;
    }
    span[type="H1"] {
        color: var(--h1-color);
        font-family: var(--h1-font);
        font-size: var(--h1-font-size);
    }
    span[type="H2"] {
        color: var(--h2-color);
        font-family: var(--h2-font);
        font-size: var(--h2-font-size);
    }
    span[type="P1"] {
        color: var(--p1-color);
        font-family: var(--p1-font);
        font-size: var(--p1-font-size);
    }
    span[type="P2"] {
        color: var(--p2-color);
        font-family: var(--p2-font);
        font-size: var(--p2-font-size);
    }
`;

const TAB_SIZE = 50;
export function getElementStyles(props: EditorElement): CSSProperties {
    const styles: CSSProperties = {};

    if (props.align === Align.Center) styles.textAlign = "center";
    else if (props.align === Align.Right) styles.textAlign = "right";

    return styles;
}

export function getLeafStyles(text: EditorText): CSSProperties {
    const styles: CSSProperties = {};

    if (text.underline) styles.textDecoration = "underline";
    if (text.italic) styles.fontStyle = "italic";
    if (text.fontSize) styles.fontSize = text.fontSize + "rem";
    if (text.color) styles.color = text.color;
    if (text.highlightColor) styles.backgroundColor = text.highlightColor;
    if (text.font) styles.fontFamily = text.font;
    if (text.weight) styles.fontWeight = text.weight;

    styles.display = "inline-block";
    styles.width = "100%";

    return styles;
}

export function getLeafProps(_text: EditorText) {
    let props: any = {};
    return props;
}

// export function augmentLeafProps(props):  {

// }

export function getRootStyles(value: WysiwygValue) {
    if (value.boxColor) {
        // no spaces outside of style element because of the `white-space: pre-wrap`
        return html`<style>
            :host {
                background-color: ${value.boxColor};
            }
        </style>`;
    } else {
        return nothing;
    }
}
