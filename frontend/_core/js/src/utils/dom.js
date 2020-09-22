//these tend to return the element they are operating on
//in order to allow a fluent/builder sort of pattern
export function appendId(parentElement, id, child) {
    const container = parentElement.querySelector(dataId(id));
    container.append(child);
    return parentElement;
}

export function prependId(parentElement, id, child) {
    const container = parentElement.querySelector(dataId(id));
    container.prepend(child);
    return parentElement;
}
export function toggleClassesId(parentElement, id, classNames, flag) {
    toggleClasses(parentElement.querySelector(dataId(id)), classNames, flag);
    return parentElement;
}

export function toggleClasses(element, classNames, flag) {
    const classList = element.classList;

    if(flag) {
        classList.add(...classNames);
    } else {
        classList.remove(...classNames);
    }
    return element;
}

export function setTextId(element, id, text) {
    const container = element.querySelector(dataId(id));
    container.innerText = text;
    return element;
}

export function setValueId(element, id, text) {
    const container = element.querySelector(dataId(id));
    container.value = text;
    return element;
}

export function dataId(id) {
    return `[data-id='${id}']`;
}
