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
    const c = Array.isArray(classNames) ? classNames : [classNames];
    toggleClasses(container, c, flag);
    return parentElement;
}

export function toggleClasses(element, classNames, flag) {
    const classList = element.classList;

    const c = Array.isArray(classNames) ? classNames : [classNames];
    if(flag) {
        classList.add(...c);
    } else {
        classList.remove(...c);
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

export function getChildId(element, id, supressError) {
    let child = element.querySelector(dataId(id));
    if(!supressError && !child) {
        console.error(`could not get child id for ${id}`);
    }

    return child;
}

export function dataId(id) {
    return `[data-id='${id}']`;
}
