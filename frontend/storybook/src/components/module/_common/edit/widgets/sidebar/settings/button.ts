import { argsToAttrs } from "@utils/attributes";
import { Kind } from "@elements/module/_common/edit/widgets/settings/button";
import "@elements/module/_common/edit/widgets/settings/button";
import "@elements/module/_common/edit/widgets/settings/bubble";
import "@elements/module/_common/edit/widgets/settings/bubble-content";
import { mapToString, arrayCount } from "@utils/array";

const kinds: Array<Kind> = [
    "attempts",
    "card-double",
    "card-single",
    "continue-all",
    "continue-click",
    "continue-some",
    "highlight",
    "highlight-off",
    "no-limit",
    "n_choices",
    "n_pairs",
    "order",
    "randomize",
    "rounds",
    "score",
    "score-off",
    "swap",
    "time-limit",
    "time-limit-off",
];

export default {
    title: "Module / _COMMON /  edit / Widgets / Sidebar / Settings ",
};

interface Args {
    kind: Kind;
    active: boolean;
    hasNumber: boolean;
    offsetContainer: boolean;
}

const DEFAULT_ARGS: Args = {
    kind: "continue-some",
    active: false,
    hasNumber: true,
    offsetContainer: true,
};

export const Button = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    const { hasNumber, offsetContainer, ...buttonProps } = props;

    if (hasNumber) {
        (buttonProps as any).num = 7;
    }

    return `
        ${
            offsetContainer
                ? `<div style="margin-left: 200px; margin-top: 100px;">`
                : ``
        }
            <module-settings-button ${argsToAttrs(buttonProps)}>
                ${renderBubble(buttonProps.kind)}
            </module-settings-button>
        ${offsetContainer ? `</div>` : ``}
    `;
};

function renderBubble(kind: Kind) {
    const content = (() => {
        switch (kind) {
            case "attempts":
            case "n_choices":
            case "n_pairs":
                return "<select><option>1</option><option>2</option></select>";

            case "time-limit":
            case "continue-some":
            case "rounds":
                return "<input type='text'></input>";

            default:
                return "";
        }
    })();

    if (content == "") {
        return "";
    } else {
        return `
            <module-settings-bubble slot="bubble">
                <module-settings-bubble-content kind="${kind}">
                    ${content}
                </module-settings-bubble-content>
            </module-settings-bubble>
        `;
    }
}

Button.args = DEFAULT_ARGS;

Button.argTypes = {
    kind: {
        control: {
            type: "inline-radio",
            options: kinds,
        },
    },
    bubble: {
        control: {
            type: "inline-radio",
            options: ["none", "text", "input", "select"],
        },
    },
};
