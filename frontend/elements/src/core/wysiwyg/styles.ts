import { css, unsafeCSS } from "lit-element";
import { CSSProperties } from "react";
import { CustomElement, CustomText } from "./slate-wysiwyg-react/EditorBackbone";
import { Align, getDefault } from "./wysiwyg-types";

export const baseStyles = css`
    p {
        margin: 0px;
        font-size: ${unsafeCSS(getDefault('fontSize'))};
        font-family: ${unsafeCSS(getDefault('font'))};
        font-weight: ${unsafeCSS(getDefault('weight'))};
    }
`;

const TAB_SIZE = 50;
export function getElementStyles(props: CustomElement) {
    let styles: CSSProperties = {};

    if(props.align === Align.Center) styles.textAlign = "center";
    else if(props.align === Align.Right) styles.textAlign = "right";

    if(props.indentCount > 0) styles.textIndent = props.indentCount * TAB_SIZE;

    return styles;
}


export function getLeafStyles(text: CustomText) {
    let styles: CSSProperties = {};

    if(text.underline) styles.textDecoration = "underline";
    if(text.italic) styles.fontStyle = "italic";
    if(text.fontSize) styles.fontSize = text.fontSize;
    if(text.color) styles.color = text.color;
    if(text.highlightColor) styles.backgroundColor = text.highlightColor;
    if(text.font) styles.fontFamily = text.font;
    if(text.weight) styles.fontWeight = text.weight;

    return styles;
}
