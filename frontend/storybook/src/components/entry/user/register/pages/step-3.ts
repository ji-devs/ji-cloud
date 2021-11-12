import { argsToAttrs } from "@utils/attributes";
import "@elements/entry/user/register/pages/step3";
import "@elements/core/buttons/rectangle";
import "@elements/core/lists/list-vertical";
import "@elements/core/lists/list-horizontal";
import "@elements/core/inputs/composed/checkbox";
import { Rectangle as RectangleButton } from "~/components/core/buttons/rectangle";
import { AFFILIATION_OPTIONS, AGE_OPTIONS, SUBJECT_OPTIONS } from "~/mock/meta";
import { mapToString } from "@utils/array";

export default {
    title: "Entry / User / Register / Pages",
};

const STR_SUBMIT = "Submit";

interface Args {}

const DEFAULT_ARGS: Args = {};

export const Step3 = (props?: Partial<Args>) => {
    props = props ? { ...DEFAULT_ARGS, ...props } : DEFAULT_ARGS;

    return `
        <page-register-step3>
            ${mapToString(AGE_OPTIONS, (label) => {
                return `<input-checkbox label="${label}" slot="ages"></input-checkbox>`;
            })}
            ${mapToString(SUBJECT_OPTIONS, (label) => {
                return `<input-checkbox label="${label}" slot="subjects"></input-checkbox>`;
            })}
            ${mapToString(AFFILIATION_OPTIONS, (label) => {
                return `<input-checkbox label="${label}" slot="affiliations"></input-checkbox>`;
            })}
            <button-rect slot="submit" color="red" size="medium">${STR_SUBMIT}</button-rect> 
        </page-register-step3>

    `;
};

Step3.args = DEFAULT_ARGS;
