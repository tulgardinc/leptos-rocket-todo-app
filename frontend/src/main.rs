use std::{collections::HashMap, str::FromStr};

use leptos::{html::Object, logging::log, wasm_bindgen::JsCast, *};
use serde::{Deserialize, Serialize};
use web_sys::{wasm_bindgen::{closure::Closure, JsValue}, Event, HtmlInputElement};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| App());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    name: String,
    is_complete: bool,
}

#[component]
fn App() -> impl IntoView {
    let (todo_input, set_todo_input) = create_signal(String::new());
    let (todos, set_todos) = create_signal(Vec::<Todo>::new());

    let todo_resource = create_resource(|| (), |_| async move {
        let resp = reqwest::get("http://127.0.0.1:8000/todos")
            .await
            .expect("failed to get todos")
            .json::<Vec<Todo>>()
            .await
            .expect("failed to get todo text");
        resp
    });

    let _ = watch( move || todo_resource.get(),
move |val, _, _| {
        match val {
            Some(inner) => {set_todos(inner.to_owned()); log!("{:?}", inner)},
            None => log!("no todos found")
        };
    }, false
    );

    let add_todo_action = create_action(move |input: &Todo| {
        let input = input.clone();
        async move {
            let client = reqwest::Client::new();
            let resp = client.post("http://127.0.0.1:8000/todos")
            .body(serde_json::to_string(&input).expect("failed to serialize todo"))
            .send()
            .await
            .expect("failed to post todo")
            .json::<Todo>()
            .await
            .expect("Failed to deserialize");

            set_todos.update(|val| (*val).push(resp));
            
        }
    });

    let handle_todo_creation = move || {
        if todo_input.get() == "" {
            return
        }

        let new_todo = Todo {
            id: None,
            name: todo_input.get(),
            is_complete: false,
        };

        add_todo_action.dispatch(new_todo);


        let input = document()
            .get_element_by_id("add-input")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        input.focus().unwrap();
        input.set_value("");
        set_todo_input.set(String::new());
    };

    let delete_todo_action = create_action(|input: &i32| {
        let id = input.clone();
        async move {
            let client = reqwest::Client::new();
            client.delete(format!("http://127.0.0.1:8000/todos/{}", id))
            .send()
            .await
            .expect("failed to delete todo");
        }
    });

    let handle_todo_delete = move |id: i32| {
        set_todos.update(|val| (*val).retain(|t| t.id.unwrap() != id) );
        delete_todo_action.dispatch(id);
    };

    let toggle_todo_action = create_action(|input: &Todo| {
        let todo = input.clone();
        async move {
            let client = reqwest::Client::new();
            client.patch("http://127.0.0.1:8000/todos")
            .json(&todo)
            .send()
            .await
            .expect("failed to delete todo");
        }
    });


    let handle_todo_done_toggle = move |id: i32| {
        let mut todo_to_change = None;
        set_todos.update(|val| {
            for todo in val.iter_mut() {
                if todo.id.unwrap() == id {
                    todo.is_complete = !todo.is_complete;
                    todo_to_change = Some(todo.clone());
                }
            }
        });
        toggle_todo_action.dispatch(todo_to_change.unwrap());
    };

    view! {
        <div class="outer">
            <div class="inner">
                <h1>Todos</h1>
                <hr/>
                <div class="list">
                    {move || {
                        todos
                            .get()
                            .into_iter()
                            .map(|t| {
                                view! {
                                    <div class="todo-item">
                                        <button
                                            on:click=move |_| {
                                                handle_todo_done_toggle(t.id.unwrap());
                                            }

                                            class=if !t.is_complete {
                                                "todo-button"
                                            } else {
                                                "todo-done-button"
                                            }
                                        >

                                            {if !t.is_complete { "TODO" } else { "DONE" }}
                                        </button>
                                        <h2 class=format!(
                                            "todo-title {}",
                                            if t.is_complete { "todo-done" } else { "" },
                                        )>{t.name}</h2>
                                        <button
                                            on:click=move |_| {
                                                handle_todo_delete(t.id.unwrap());
                                            }

                                            class="delete-button"
                                        >
                                            DELETE
                                        </button>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}

                </div>
                <div class="new-todo-container">
                    <input
                        value=todo_input.get()
                        id="add-input"
                        type="text"
                        class="todo-name"
                        on:input=move |e| {
                            set_todo_input
                                .set(
                                    e
                                        .target()
                                        .unwrap()
                                        .dyn_into::<HtmlInputElement>()
                                        .unwrap()
                                        .value(),
                                )
                        }

                        on:keydown=move |e| {
                            if e.key() == "Enter" {
                                handle_todo_creation();
                            }
                        }
                    />

                    <button
                        on:click=move |_| {
                            handle_todo_creation();
                        }

                        class="create"
                    >
                        ADD
                    </button>
                </div>
            </div>
        </div>
    }
}
