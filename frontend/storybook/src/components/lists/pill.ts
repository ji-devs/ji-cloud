import "@elements/lists/pill";
import "@elements/inputs/checkbox";



export default {
  title: 'Lists/Pill',
}



export const PillListItem = ({label}) => {
    return `
       <pill-listitem label=${label}></pill-listitem>
    `;
}

PillListItem.args = {
    label: "School"
};
