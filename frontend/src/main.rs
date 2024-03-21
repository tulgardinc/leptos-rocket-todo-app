use leptos::{logging::log, *};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| App());
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="outer">
            <div class="inner">
                <h1>Todos</h1>
                <hr/>
                <div class="list">
                    <div class="todo-item">
                        <button class="todo-button">TODO</button>
                        <h2 class="todo-title">This is a todo</h2>
                        <button class="delete-button">DELETE</button>
                    </div>
                </div>
                <div class="new-todo-container">
                    <input type="text" class="todo-name"/>
                    <button on:click=|_| { log!("yo") } class="create">
                        ADD
                    </button>
                </div>
            </div>
        </div>
    }
}
