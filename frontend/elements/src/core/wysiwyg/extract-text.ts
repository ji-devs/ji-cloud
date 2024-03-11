import { EditorElement, EditorText } from "./slate-wysiwyg-react/EditorBackbone";
import { WysiwygValue } from "./wysiwyg-types";

// Extracts the text from a wysiwyg value.
function extract_wysiwyg_text(wysiwyg_value: string): string {
    let value: WysiwygValue = JSON.parse(wysiwyg_value);
    let output = value.content.map((child: EditorElement) => {
        return child.children.map((child: EditorText) => {
            return child.text ?? "";
        }).join("");
    }).join(" ");
    return output;
}

// @ts-ignore
window.extract_wysiwyg_text = extract_wysiwyg_text;
