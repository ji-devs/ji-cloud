import React, { ReactElement, useCallback, useState } from "react";
import { Descendant } from "slate";
import { Editable, Slate } from "slate-react";
import { Leaf } from "./Leaf";
import { Element } from "./Element";
import { EditorBackbone } from "./EditorBackbone";

interface Props {
    backbone: EditorBackbone;
    value: Descendant[],
    onChange: (value: Descendant[]) => void;
    onBlur: () => void,
}

export function EditorComponent(props: Props): ReactElement {
    const [value, setValue]: [Descendant[], any] = useState(props.value);

    const renderElement = useCallback(props => {
        return React.createElement(Element, {...props});
    }, []);

    const renderLeaf = useCallback(props => {
        return React.createElement(Leaf, {...props});
    }, []);

    const onChange = (value: Descendant[]) => {
        setValue(value);
        props.onChange(value);
    }

    return React.createElement(
        Slate,
        {
            editor: props.backbone.editor,
            value,
            onChange,
            children: [React.createElement(
                Editable,
                {
                    key: "Editable",
                    renderElement,
                    renderLeaf,
                    onBlur: props.onBlur,
                    onKeyDown: event => {
                        if (!event.ctrlKey) {
                            return
                        }

                        if(props.backbone.keyMaps.has(event.key)) {
                            event.preventDefault();
                            const func = props.backbone.keyMaps.get(event.key)!;
                            func();
                        }
                    }
                }
            )]
        }
    );
}
