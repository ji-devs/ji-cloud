import "@elements/core/cards/grey";

export default {
  title: 'Core / Cards',
}

interface Args {
    width: number;
    height: number;
    contents: string
}

const DEFAULT_ARGS:Args = {
    width: 300,
    height: 300,
    contents: "hello"
}

export const Grey = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {width, height, contents} = props; 

    return `
        <div style="width: ${width}px; height: ${height}px">
            <card-grey>${contents}</card-grey>
        </div>
    `;

}

Grey.args = DEFAULT_ARGS;