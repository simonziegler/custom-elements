// Create a class for the element
class tf extends HTMLElement {
    // Specify observed attributes so that
    // attributeChangedCallback will work
    static get observedAttributes() {
      return ["color", "size"];
    }
  
    constructor() {
      // Always call super first in constructor
      super();
  
      const shadow = this.attachShadow({ mode: "open" });
  
      const div = document.createElement("div");
      const style = document.createElement("style");
      shadow.appendChild(style);
      shadow.appendChild(div);
    }
  
    connectedCallback() {
      console.log("Custom tf element added to page.");
      updateStyle(this);
    }
  
    disconnectedCallback() {
      console.log("Custom tf element removed from page.");
    }
  
    adoptedCallback() {
      console.log("Custom tf element moved to new page.");
    }
  
    attributeChangedCallback(name, oldValue, newValue) {
      console.log("Custom tf element attributes changed.");
      updateStyle(this);
    }
  }
  
  customElements.define("custom-tf", tf);
  
  function updateStyle(elem) {
    const shadow = elem.shadowRoot;
    shadow.querySelector("style").textContent = `
      div {
        width: ${elem.getAttribute("size")}px;
        height: ${elem.getAttribute("size")}px;
        background-color: ${elem.getAttribute("color")};
      }
    `;
  }
  
  const add = document.querySelector(".add");
  const update = document.querySelector(".update");
  const remove = document.querySelector(".remove");
  let ceTextField;
  
  update.disabled = true;
  remove.disabled = true;
  
  function random(min, max) {
    return Math.floor(Math.random() * (max - min + 1) + min);
  }
  
  add.onclick = function () {
    // Create a custom tf element
    ceTextField = document.createElement("ce-text-field");
    ceTextField.setAttribute("label", "Label");
    document.body.appendChild(ceTextField);

    ceTextField.addEventListener('change', (event) => {
      console.log('button.js: Text Field changed:', event.detail);
      console.log('button.js: Text Field value:', ceTextField.value);
    });
  
    update.disabled = false;
    remove.disabled = false;
    add.disabled = true;
  };
  
  update.onclick = function () {
    // Randomly update text field's attributes
    ceTextField.setAttribute("label", "New Label");
  };
  
  remove.onclick = function () {
    // Remove the text field
    document.body.removeChild(ceTextField);
  
    update.disabled = true;
    remove.disabled = true;
    add.disabled = false;
  };