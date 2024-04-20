import { For, createSignal, createResource } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

type Todo = {
  id: number;
  value: string;
};

function App() {
  const [todo, setTodo] = createSignal("");

  const fetcher = () => invoke("todos_get") as Promise<Array<Todo>>;
  const [data, { mutate }] = createResource(fetcher);

  async function addTodo() {
    if (!todo()) {
      return;
    }

    try {
      const res = (await invoke("todo_add", { value: todo() })) as Todo;
      const newTodo = {
        id: res.id,
        value: res.value,
      };
      mutate([...(data() ?? []), newTodo]);
    } catch (e) {
      console.error(e);
    }

    clearInput();
  }

  function clearInput() {
    setTodo("");
  }

  return (
    <div class="container">
      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          addTodo();
        }}
      >
        <input
          id="todo-input"
          value={todo()}
          onChange={(e) => setTodo(e.currentTarget.value)}
          placeholder="Enter a todo..."
        />
        <button id="clear-input" type="button" onclick={clearInput}>
          X
        </button>
        <button type="submit">Add</button>
      </form>

      <div class="row" id="todo-list">
        <ul>
          <For each={data()}>
            {(todo) => (
              <li>
                {todo.id}: {todo.value}
              </li>
            )}
          </For>
        </ul>
      </div>
    </div>
  );
}

export default App;
