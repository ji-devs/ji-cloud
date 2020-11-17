//these tend to return the element they are operating on
//in order to allow a fluent/builder sort of pattern
export function appendId(parentElement, id, child) {
    const container = getChildId(parentElement, id);
    container.append(child);
    return parentElement;
}

export function prependId(parentElement, id, child) {
    const container = getChildId(parentElement, id);
    container.prepend(child);
    return parentElement;
}
export function toggleClassesId(parentElement, id, classNames, flag) {
    const container = getChildId(parentElement, id);
    toggleClasses(container, classNames, flag);
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
    const container = getChildId(element, id);
    container.innerText = text;
    return element;
}

export function appendTextId(element, id, text) {
    const container = getChildId(element, id);
    container.textContent += text;
    return element;
}
export function appendTextLineId(element, id, text) {
    const container = getChildId(element, id);
    container.textContent += text + '\n';
    return element;
}
export function setValueId(element, id, text) {
    const container = getChildId(element, id);
    container.value = text;
    return element;
}

export function appendValueLineId(element, id, text) {
    const container = getChildId(element, id);
    container.value += text + '\n';
    return element;
}
export function appendValueId(element, id, text) {
    const container = getChildId(element, id);
    container.value += text;
    return element;
}

export function setAttributeId(element, id, attr, value) {
    const container = getChildId(element, id);
    container.setAttribute(attr, value);
    return element;
}

export function getChildId(element, id) {
    return element.querySelector(dataId(id));
}

export function dataId(id) {
    return `[data-id='${id}']`;
}
