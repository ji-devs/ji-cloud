export const storyAbout = (name, component, about) => {
    if(!about) {
        throw new Error("if calling storyAbout, must supply some markdown!");
    }

    const base = story(name, component);
    base.story.parameters.notes.about = about;
    return base;
}

export const story = (name, component) => {
    component.story = {
        name,
        parameters: {notes: {}}
    }
    return component;
}
