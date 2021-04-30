import React from "react";
import { CSSProperties, ReactElement } from "react";
import { RenderLeafProps } from "slate-react";
import { CustomText } from "./EditorBackbone";

function getStyles(text: CustomText) {
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

export function Leaf(props: RenderLeafProps): ReactElement {
    return React.createElement(
        "span",
        {
            style: getStyles(props.leaf),
            ...props.attributes,
            children: props.children,
        }
    );
}
