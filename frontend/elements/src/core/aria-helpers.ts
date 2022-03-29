/// Improves keyboard accessibility for elements which can be clicked.
///
/// See: https://www.w3.org/WAI/GL/wiki/Making_actions_keyboard_accessible_by_using_keyboard_event_handlers_with_WAI-ARIA_controls
export const applyButtonRoleEvents = (element: EventTarget) => {
    console.log('applyRoles', element);

    element.addEventListener("keydown", evt => keydownHandler(<KeyboardEvent> evt));
    element.addEventListener("keyup", evt => keyupHandler(<KeyboardEvent> evt));
}

let clickHandler = (evt: Event) => {
    evt.preventDefault();
    evt.target?.dispatchEvent(new Event("click"));
};

const KEY_ENTER = "Enter";
const KEY_SPACE = "Space";

let keydownHandler = (evt: KeyboardEvent) => {
    if (evt.code === KEY_SPACE) {
        evt.preventDefault();
    }
}

let keyupHandler = (evt: KeyboardEvent): void => {
    if (evt.code === KEY_ENTER || evt.code === KEY_SPACE) {
        clickHandler(evt);
    }
}
