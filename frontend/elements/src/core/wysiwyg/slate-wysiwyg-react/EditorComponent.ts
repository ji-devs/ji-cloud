import React, { ReactElement, FocusEvent } from "react";
import { Descendant } from "slate";
import {
    Editable,
    RenderElementProps,
    RenderLeafProps,
    Slate,
} from "slate-react";
import { Leaf } from "./Leaf";
import { Element } from "./Element";
import { EditorBackbone } from "./EditorBackbone";

interface Props {
    backbone: EditorBackbone;
    value: Descendant[];
    onChange: (value: Descendant[]) => void;
    onBlur: (e: FocusEvent) => void;
}

interface State {
    value: Descendant[];
}

export class EditorComponent extends React.Component<Props, State> {
    public state = {
        value: this.props.value,
    };

    private renderLeaf(props: RenderLeafProps) {
        return React.createElement(Leaf, { ...props });
    }

    private renderElement(props: RenderElementProps) {
        return React.createElement(Element, { ...props });
    }

    private onChange(value: Descendant[]) {
        this.setState({ value });
        this.props.onChange(value);
    }

    public setValue(value: Descendant[]) {
        this.setState({ value });
    }

    public render(): ReactElement {
        return React.createElement(Slate, {
            editor: this.props.backbone.editor,
            value: this.state.value,
            onChange: (e: any) => this.onChange(e),
            children: [
                React.createElement(Editable, {
                    key: "Editable",
                    renderElement: this.renderElement,
                    renderLeaf: this.renderLeaf,
                    onBlur: this.props.onBlur,
                    onKeyDown: (event) => {
                        if (!event.ctrlKey) {
                            return;
                        }

                        if (this.props.backbone.keyMaps.has(event.key)) {
                            event.preventDefault();
                            const func = this.props.backbone.keyMaps.get(
                                event.key
                            )!;
                            func();
                        }
                    },
                }),
            ],
        });
    }
}
