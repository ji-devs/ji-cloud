import React, { CSSProperties, ReactElement } from "react";
import { CustomElement } from "./EditorBackbone";
import { Align, ElementType } from "../wysiwyg-types";
import { RenderElementProps } from "slate-react";

const TAB_SIZE = 50;

function getStyles(props: CustomElement) {
    let styles: CSSProperties = {};

    if(props.align === Align.Center) styles.textAlign = "center";
    else if(props.align === Align.Right) styles.textAlign = "right";

    if(props.indentCount > 0) styles.textIndent = props.indentCount * TAB_SIZE;

    return styles;
}


export function Element(props: RenderElementProps): ReactElement {
    const styles = getStyles(props.element);

    switch (props.element.element) {
        case ElementType.H1:
            return React.createElement("h1", {style: styles, ...props});
        case ElementType.H2:
            return React.createElement("h2", {style: styles, ...props});
        case ElementType.P2:
            return React.createElement("p", {p2:"", style: styles, ...props});
        default: // ElementType.P1:
            return React.createElement("p", {p1:"", style: styles, ...props});
    }

}
