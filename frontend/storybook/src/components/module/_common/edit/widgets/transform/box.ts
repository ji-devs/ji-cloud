import {argsToAttrs} from "@utils/attributes";
import "@elements/module/_common/edit/widgets/transform/box";

export default {
    title: "Module / _COMMON /  edit /Widgets / Transform"
}

interface Args {
    width: number;
    height: number;
}

const DEFAULT_ARGS:Args = {
    width: 300,
    height: 100,
}

export const Box = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {width, height, ...boxProps} = props

    let positionStyle = `position: absolute;`;
    positionStyle += ` left: calc((100vw - ${width}px)/2);`;
    positionStyle += ` top: 200px;`

    let style = positionStyle;

    let contentStyle = ``; 
    contentStyle += ` display: flex;`
    contentStyle += ` width: ${width}px;`;
    contentStyle += ` height: ${height}px;`;
    contentStyle += ` justify-content: center;`;
    contentStyle += ` align-items: center;`;
    return `
    <transform-box menuButtonVisible active style="${style}" width="${width}" height="${height}" ${argsToAttrs(boxProps)}>
        <div style="${contentStyle}">
        Contents Here

        </div>
        <button-icon slot="menu-btn" id="button" icon="circle-kebab-grey"></button-icon>
    </transform-box>
    `;
}

Box.args = DEFAULT_ARGS;
