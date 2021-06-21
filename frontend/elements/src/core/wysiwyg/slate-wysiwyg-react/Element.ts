import React, { ReactElement } from "react";
import { RenderElementProps } from "slate-react";
import { getElementStyles } from "../styles";


export function Element(props: RenderElementProps): ReactElement {
    const styles = getElementStyles(props.element);

    const type = props.element.element;

    return React.createElement("p", {style: styles, type, ...props});
}
