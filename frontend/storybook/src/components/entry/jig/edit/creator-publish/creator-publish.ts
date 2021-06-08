import "@elements/entry/jig/edit/publish/publish";
import "@elements/entry/jig/edit/publish/add-resource";
import "@elements/entry/jig/edit/publish/add-resource-method";
import "@elements/core/inputs/switch";
import "@elements/core/inputs/text";
import "@elements/core/inputs/form/textarea";
import "@elements/core/inputs/dropdowns/dropdown-select";
import "@elements/core/buttons/rectangle";
import "@elements/core/pills/pill-close";

import { argsToAttrs } from "@utils/attributes";

export default {
    title: "Entry / Jig / Edit / Publish"
}

interface Args {
    recentCount: number;
}

const DEFAULT_ARGS:Args = {
    recentCount: 12,
}

export const Publish = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <div style="padding:30px;background-color:#d7eafa;">
            <jig-edit-publish ${argsToAttrs(props)}>
                <img-ji slot="img" lib="mock" size="full" id="jig-gallery.jpg"></img-ji>
                <input-switch slot="public" label="My JIG is public"></input-switch>

                
                <input-text
                    slot="name"
                    label="JIG’s name"
                    value="Parashat Bereshit"
                ></input-text>
                <input-form-textarea
                    slot="description"
                    label="Description"
                    value="This game is about… using … Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry’s standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."
                ></input-form-textarea>

                <dropdown-select
                    slot="language"
                    label="Language of instructions"
                    value="English"
                ></dropdown-select>
                <dropdown-select
                    slot="age"
                    label="Age"
                    value="All Ages"
                ></dropdown-select>
                <dropdown-select
                    slot="goal"
                    label="Teaching Goal"
                    placeholder="Select from the list"
                ></dropdown-select>
                <dropdown-select
                    slot="catagories-select"
                    label="Categories"
                    placeholder="Select from the list"
                ></dropdown-select>
                <pill-close
                    slot="category-labels"
                    label="Hebrew"
                ></pill-close>
                <pill-close
                    slot="category-labels"
                    label="Letters"
                ></pill-close>
                <pill-close
                    slot="category-labels"
                    label="Letter recognition"
                ></pill-close>
                <pill-close
                    slot="category-labels"
                    label="Holidays"
                ></pill-close>
                <pill-close
                    slot="category-labels"
                    label="Passover"
                ></pill-close>
                <pill-close
                    slot="category-labels"
                    label="Jewish Texts"
                ></pill-close>
                <pill-close
                    slot="category-labels"
                    label="Torah/Tanach/Bible/Chumash"
                ></pill-close>


                <jig-edit-publish-add-resource
                    slot="additional-resources"
                    label="Add Lesson Plan"
                >
                    <jig-edit-publish-add-resource-method
                        slot="add-method"
                        kind="upload"
                    ></jig-edit-publish-add-resource-method>
                    <jig-edit-publish-add-resource-method
                        slot="add-method"
                        kind="link"
                    ></jig-edit-publish-add-resource-method>
                </jig-edit-publish-add-resource>
                <jig-edit-publish-add-resource
                    slot="additional-resources"
                    label="Add Curriculum"
                ></jig-edit-publish-add-resource>
                <jig-edit-publish-add-resource
                    slot="additional-resources"
                    label="Add Activities Ideas"
                ></jig-edit-publish-add-resource>
                <jig-edit-publish-add-resource
                    slot="additional-resources"
                    label="Add Link"
                ></jig-edit-publish-add-resource>


                <button-rect slot="publish" iconAfter="rocket">Publish JIG</button-rect>
            </jig-edit-publish>
        </div>
    `;
}

Publish.args = DEFAULT_ARGS;
