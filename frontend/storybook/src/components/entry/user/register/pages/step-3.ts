import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/register/pages/step3";
import "@elements/core/titles/ji";
import "@elements/core/dividers/or-divider";
import "@elements/core/buttons/rectangle";
import "@elements/core/lists/list-vertical";
import {Rectangle as RectangleButton} from "~/components/core/buttons/rectangle";
import {AFFILIATION_OPTIONS, AGE_OPTIONS} from "~/mock/meta";
import {mapToString} from "@utils/array";

export default {
  title: 'Entry / User / Register / Pages',
}

const STR_TITLE = "Sign Up - Step 3";
const STR_SUBTITLE = "We want to tailor the content that you find to your interests and needs.";
const STR_SUBSUBTITLE = "You can select as many as you like now and edit it later it in your profile page";
const STR_AGE_LABEL = "Which age group are you interested in?";
const STR_AFFILIATION_LABEL = "Content from which streams of Judaism do you want to see?";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const Step3 = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-register-step3 title="${STR_TITLE}">
            <title-ji slot="subtitle" size="subMedium">${STR_SUBTITLE}</title-ji>
            <title-ji slot="subtitle" size="subMedium">${STR_SUBSUBTITLE}</title-ji>
            <card-grey slot="main">
                <list-horizontal label="${STR_AGE_LABEL}">
                ${mapToString(AGE_OPTIONS, label => {
                    return `<input-checkbox label="${label}"></input-checkbox>`
                })}
                </list-horizontal>
            </card-grey>
            <card-grey slot="main">
                <list-vertical label="${STR_AFFILIATION_LABEL}">
                ${mapToString(AFFILIATION_OPTIONS, label => {
                    return `<input-checkbox label="${label}"></input-checkbox>`
                })}
                </list-vertical>
            </card-grey>

            <div slot="submit">${RectangleButton()}</div>
        </page-register-step3>

    `
}

Step3.args = DEFAULT_ARGS;