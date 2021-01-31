import React, { useState, Dispatch, SetStateAction } from 'react';
import { useForm } from "react-hook-form";
import { API_URL } from "./utils/config"
import "./App.css"

type Todo = {
  task: string
}

export interface ITodos {
  todos: Array<Todo>
}

type TodoFormProps = {
  todoContainers: ITodos,
  setTodos: Dispatch<SetStateAction<ITodos>>
}

type TodoListProps = {
  todoContainers: ITodos,
}

const TodoList: React.FC<TodoListProps> = ({todoContainers}) => {
  const todoComponents = todoContainers.todos.map((value, index) => <li key={index}>TODO: {value.task}</li>);

  return (
    <div>
      {todoComponents}
    </div>
  )
}

const TodoForm: React.FC<TodoFormProps> = ({todoContainers, setTodos}) => {
  const { register, handleSubmit, errors } = useForm<Todo>();
  const onSubmit = (todo: Todo) => {
    setTodos({"todos": todoContainers.todos.concat(todo)});
  };

  const todoMaxLength = 140;

  return (
    <div>
      <form onSubmit={handleSubmit(onSubmit)}>
        <div className="field">
          <label htmlFor="todoInput">TODO: </label>
          <input 
           name="task"
           type="text" 
           id="task" 
           ref={register({ required: true, maxLength: todoMaxLength } )}>
          </input>
          {errors.task && errors.task.type === "required"  && (
            <div className="error">You must enter TODO.</div>
          )}
          {errors.task && errors.task.type === "maxLength"  && (
            <div className="error">TODO cant be longer than {todoMaxLength}.</div>
          )}
        </div>
        <button type="submit">Submit TODO</button>
      </form>
    </div>
  )
}

const BodyHeader: React.FC = () => (
  <div>
    <h1>hy-kube project!</h1>
  </div>
)

const DailyImage: React.FC = () => {
  const photoApiUrl = API_URL + "/daily_photo";
  return (
    <div>
      <img src={photoApiUrl} className="dailyPhoto" alt="dailyPhoto"/>
    </div>
  )
}

function App() {
  const [todos, setTodos] = useState<ITodos>({"todos": []});

  return (
    <div className="App">
      <header className="App-header">
      </header>
        <BodyHeader />
        <DailyImage />
        <br />
        <TodoForm todoContainers={todos} setTodos={setTodos}/>
        <br />
        <TodoList todoContainers={todos}/>
    </div>
  );
}

export default App;
