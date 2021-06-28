import { LitElement, html, css, customElement, property } from "lit-element";
export type Size = "memory" | "flashcards" | "quiz-option" | "quiz-target" | "matching";

export const cardStyles = [css`
  :host {
    --img-padding: 10rem;
  }

  :host([size="matching"]), :host([size="quiz-option"]) {
    --card-size: 253rem;
    --border-size: 4.75rem;
  }

  :host([size="flashcards"]) {
    --card-size: 500rem;
    --border-size: 16rem;
  }

  :host([size="memory"]) {
    --card-size: 188rem;
    --border-size: 3rem;
  }

  :host([size="quiz-target"]) {
    --card-size: 431rem;
    --border-size: 4.75rem;
  }

  :host {
    cursor: pointer;
  }

  section {
    width: var(--card-size);
    height: var(--card-size);
  }

  
  .content {
    border-radius: 16px;
    border-style: solid;
    border-width: var(--border-size);
    background-color: white;

    box-sizing: border-box;
    width: 100%;
    height: 100%;

    display: flex;
    justify-content: center;
    align-items: center;
  }
`];
