import init, { BlogApp } from "./pkg/blog_wasm.js";

let app;

async function main() {
    await init();

    app = new BlogApp();

    bindEvents();
    updateAuthStatus();
    await loadPosts();
}

function bindEvents() {
    document
        .getElementById("register-form")
        .addEventListener("submit", register);

    document
        .getElementById("login-form")
        .addEventListener("submit", login);

    document
        .getElementById("logout-button")
        .addEventListener("click", logout);

    document
        .getElementById("create-post-form")
        .addEventListener("submit", createPost);

    document
        .getElementById("reload-posts-button")
        .addEventListener("click", loadPosts);
}

async function register(event) {
    event.preventDefault();

    const username =
        document.getElementById("register-username").value;

    const email =
        document.getElementById("register-email").value;

    const password =
        document.getElementById("register-password").value;

    try {
        const response =
            await app.register(username, email, password);

        showStatus(
            `Пользователь ${response.user.username} зарегистрирован`
        );

        updateAuthStatus();
    } catch (error) {
        showError(error);
    }
}

async function login(event) {
    event.preventDefault();

    const username =
        document.getElementById("login-username").value;

    const password =
        document.getElementById("login-password").value;

    try {
        const response =
            await app.login(username, password);

        showStatus(
            `Выполнен вход: ${response.user.username}`
        );

        updateAuthStatus();
    } catch (error) {
        showError(error);
    }
}

function logout() {
    try {
        app.logout();
        showStatus("Вы вышли из системы");
        updateAuthStatus();
    } catch (error) {
        showError(error);
    }
}

async function createPost(event) {
    event.preventDefault();

    const title =
        document.getElementById("post-title").value;

    const content =
        document.getElementById("post-content").value;

    try {
        const post =
            await app.create_post(title, content);

        showStatus(`Пост ${post.id} создан`);

        document.getElementById("create-post-form").reset();

        await loadPosts();
    } catch (error) {
        showError(error);
    }
}

async function loadPosts() {
    try {
        const posts = await app.load_posts();
        renderPosts(posts);
    } catch (error) {
        showError(error);
    }
}

function renderPosts(posts) {
    const container = document.getElementById("posts");
    container.innerHTML = "";

    if (posts.length === 0) {
        container.textContent = "Посты не найдены";
        return;
    }

    for (const post of posts) {
        const article = document.createElement("article");

        const title = document.createElement("h3");
        title.textContent = post.title;

        const content = document.createElement("p");
        content.textContent = post.content;

        const metadata = document.createElement("small");
        metadata.textContent =
            `ID: ${post.id}, автор: ${post.author_id}`;

        const editButton = document.createElement("button");
        editButton.type = "button";
        editButton.textContent = "Редактировать";
        editButton.addEventListener("click", () => editPost(post));

        const deleteButton = document.createElement("button");
        deleteButton.type = "button";
        deleteButton.textContent = "Удалить";
        deleteButton.addEventListener("click", () => deletePost(post.id));

        article.append(
            title,
            content,
            metadata,
            document.createElement("br"),
            editButton,
            deleteButton
        );

        container.appendChild(article);
    }
}

async function editPost(post) {
    const title = window.prompt(
        "Новый заголовок",
        post.title
    );

    if (title === null) {
        return;
    }

    const content = window.prompt(
        "Новое содержание",
        post.content
    );

    if (content === null) {
        return;
    }

    try {
        const updatedPost = await app.update_post(
            post.id,
            title,
            content
        );

        showStatus(`Пост ${updatedPost.id} обновлён`);
        await loadPosts();
    } catch (error) {
        showError(error);
    }
}

async function deletePost(id) {
    const confirmed = window.confirm(
        `Удалить пост ${id}?`
    );

    if (!confirmed) {
        return;
    }

    try {
        await app.delete_post(id);

        showStatus(`Пост ${id} удалён`);
        await loadPosts();
    } catch (error) {
        showError(error);
    }
}

function updateAuthStatus() {
    const authenticated = app.is_authenticated();

    document.getElementById("logout-button").disabled =
        !authenticated;

    document.getElementById("create-post-form").hidden =
        !authenticated;
}

function showStatus(message) {
    const status = document.getElementById("status");
    status.textContent = message;
}

function showError(error) {
    const message =
        error instanceof Error ? error.message : String(error);

    showStatus(`Ошибка: ${message}`);
}

main().catch(showError);