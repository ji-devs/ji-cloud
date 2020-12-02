
import {renderTemplate as tmpl} from "@utils/template";
import successTmpl from "@templates/module/memory/todo_temp_ref/memory-success.html";
import summaryTmpl from "@templates/module/memory/todo_temp_ref/memory-summary.html";
import duplicate3Tmpl from "@templates/module/memory/todo_temp_ref/duplicate/memory-step-three.html";
import duplicate4Tmpl from "@templates/module/memory/todo_temp_ref/duplicate/memory-step-four.html";
import wordsAndImages1Tmpl from "@templates/module/memory/todo_temp_ref/images/memory-images-one.html";
import wordsAndImages2Tmpl from "@templates/module/memory/todo_temp_ref/images/memory-images-two.html";
import wordsAndImagesTooltipTmpl from "@templates/module/memory/todo_temp_ref/images/added-image-tooltip.html";



export default {
  title: 'Modules/Memory/Todo/Reference (temp)',
}

export const Success = () => tmpl(successTmpl);
export const Summary = () => tmpl(summaryTmpl);
export const Duplicate_Step_3 = () => tmpl(duplicate3Tmpl);
export const Duplicate_Step_4 = () => tmpl(duplicate4Tmpl);
export const Words_and_Images_Step_1_Text = () => tmpl(wordsAndImages1Tmpl);
export const Words_and_Images_Step_1_Images = () => tmpl(wordsAndImages2Tmpl);
export const Words_and_Images_Tooltip = () => tmpl(wordsAndImagesTooltipTmpl);