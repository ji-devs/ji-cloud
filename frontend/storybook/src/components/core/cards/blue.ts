import "@elements/core/cards/blue";

export default {
  title: 'Core / Cards',
}

interface Args {
    width: number;
    height: number;
    contents: string;
}

const DEFAULT_ARGS:Args = {
    width: 300,
    height: 300,
    contents: "hello"
}

export const Blue = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {width, height, contents} = props; 

    return `
        <div style="width: ${width}px; height: ${height}px">
            <card-blue>${contents}</card-blue>
        </div>
    `;

}

Blue.args = DEFAULT_ARGS;