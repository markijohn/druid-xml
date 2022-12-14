
//Flex
window.customElements.define('druid-flex', class extends HTMLElement {
	constructor() {
		super();

		//flex container attribute
		this.style.display = "flex";
		//direction
		console.log("druid flex direction.. ", this.getAttribute("direction"));
		this.style.flexDirection = this.getAttribute("direction");
	}
});

//textbox
window.customElements.define('druid-textbox', class extends HTMLElement {
	constructor() {
		super();
		this.type = "input";
		this.style.flex = this.getAttribute("flex");
		let input = document.createElement("input");
		input.style = this.style;
		this.appendChild(input);
	}
});

//button
window.customElements.define('druid-button', class extends HTMLElement {
	constructor() {
		super();
		this.style.flex = this.getAttribute("flex");
		let btn = document.createElement("button");
		btn.textContent = this.textContent;
		this.textContent = '';
		this.appendChild(btn);
	}
});