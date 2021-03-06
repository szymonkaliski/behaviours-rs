import ex01 from "./01.js";
import ex02 from "./02.js";
import ex03 from "./03.js";
import ex04 from "./04.js";
import ex05 from "./05.js";

const examples = {
  "1": [ex01, "basic repulsion"],
  "2": [ex02, "multiple point attraction"],
  "3": [ex03, "diffusion-limited aggregation"],
  "4": [ex04, "basic 3d example"],
  "5": [ex05, "diffusion-limited aggregation in 3d"]
};

const searchValue = document.location.search.replace("?", "");

if (!searchValue) {
  const wrapper = document.createElement("div");

  wrapper.innerHTML = Object.keys(examples)
    .map(key => {
      const note = examples[key][1];
      return `<div><a href="${
        document.location.href
      }?${key}">${key} &mdash; ${note}</a></div>`;
    })
    .join("\n");

  wrapper.style.fontFamily = "sans-serif";
  wrapper.style.fontSize = "18px";

  document.body.appendChild(wrapper);
} else {
  examples[searchValue][0]();
}
