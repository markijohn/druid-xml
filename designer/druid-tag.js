
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

window.onload = () => {
	let custom_parts = document.querySelectorAll("custom");
	custom_parts.forEach( cpart => {
		let html = cpart.innerHTML;
		let tag = cpart.getAttribute("map");
		if( tag && tag.trim() != "" ) {
			let custom_impls = document.querySelectorAll( tag );
			custom_impls.forEach( ci => {
				//let kv = ci.getAttributeNames().reduce( (kv,k) => { kv[k] = ci.getAttribute(k); return kv } , {} );
				let build_html = html.replace(/\$\{([a-z|A-Z|0-9|_]+)?\}/g, function(match, item) {
					//return kv[item] || ""
					return ci.getAttribute( item ) || ""
				} );
				ci.outerHTML = build_html;
			});
		}
	});
}