import { css, unsafeCSS } from "lit-element";
import { CSSProperties } from "react";
import { CustomElement, CustomText } from "./slate-wysiwyg-react/EditorBackbone";
import { Align } from "./wysiwyg-types";

export const baseStyles = css`
    p {
        margin: 0px;
    }
    p[type=H1] {
        color: var(--h1-color);
        font-family: var(--h1-font);
        font-size: var(--h1-font-size);
    }
    p[type=H2] {
        color: var(--h2-color);
        font-family: var(--h2-font);
        font-size: var(--h2-font-size);
    }
    p[type=P1] {
        color: var(--p1-color);
        font-family: var(--p1-font);
        font-size: var(--p1-font-size);
    }
    p[type=P2] {
        color: var(--p2-color);
        font-family: var(--p2-font);
        font-size: var(--p2-font-size);
    }
`;

const TAB_SIZE = 50;
export function getElementStyles(props: CustomElement) {
    let styles: CSSProperties = {};

    if(props.align === Align.Center) styles.textAlign = "center";
    else if(props.align === Align.Right) styles.textAlign = "right";

    if(props.indentCount > 0) styles.textIndent = (props.indentCount * TAB_SIZE) + 'px';

    return styles;
}


export function getLeafStyles(text: CustomText) {
    let styles: CSSProperties = {};

    if(text.underline) styles.textDecoration = "underline";
    if(text.italic) styles.fontStyle = "italic";
    if(text.fontSize) styles.fontSize = text.fontSize + 'px';
    if(text.color) styles.color = text.color;
    if(text.highlightColor) styles.backgroundColor = text.highlightColor;
    if(text.font) styles.fontFamily = text.font;
    if(text.weight) styles.fontWeight = text.weight;

    return styles;
}
