export function renderTemplate(template, data?: any) {
    const fragment = renderTemplateFragment(template, data);
    return fragment.firstElementChild;
}

export function renderTemplateFragment(template, data?: any) {
    const str = renderTemplateString(template, data);
    const el = document.createElement("template");
    el.innerHTML = str;
    return el.content;
}

export function renderDivText(text) {
	return renderTemplate(`<div>${text}</div>`, {text});
}

//Very simple replacement of `${pattern}`
//Probably not safe, but only used for storybook / internal dev anyway
export function renderTemplateString(template, data) {
    if(data == null) {
            return template;
    } else {
            return interpolate(template, data);
    }
}

//See: https://stackoverflow.com/a/47358102
const regex = /\${[^{]+}/g;

export function interpolate(template, variables, fallback?:any) {
    return template.replace(regex, (match) => {
        const path = match.slice(2, -1).trim();
        return getObjPath(path, variables, fallback);
    });
}

//get the specified property or nested property of an object
function getObjPath(path, obj, fallback = '') {
    return path.split('.').reduce((res, key) => res[key] || fallback, obj);
}
