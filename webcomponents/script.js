class UserCard extends HTMLElement {
    connectedCallback() {
        const name = this.getAttribute('name');
        const picture_url = this.getAttribute('picture');
        const description = this.getAttribute('description');

        this.innerHTML = `
            <div class="user-card">
                <style>
                    --picture-url: ${picture_url}
                </style>
                <h2>${name}</h2>
                <img src="${picture_url}" alt="${name}">
                <p>${description}</p>
            </div>
        `;
    }
}

customElements.define('user-card', UserCard);
