import styles from './list.module.css'
import NewItem from "../icons/newItem";
import {useEffect, useState} from "react";
import {DoneItem} from "../icons/doneItem";
import {RemoveItem} from "../icons/removeItem";

function updateStatus(name, status) {
    const data = {name, status}
    fetch('http://localhost:8080/todo/create', {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    })
        .then()
        .catch( () =>  console.log("err"));
}

function deleteTask(key) {
    fetch('http://localhost:8080/todo/delete?key='+key, {method: "DELETE"})
        .then()
        .catch( () =>  console.log("err"));
}

function reloadPage() {
    window.location.reload()
}

export default function List() {
    const [data, setData] = useState(null)
    const [isLoading, setLoading] = useState(false)
    const [input, setInput] = useState("");

    const inputHandler = e => {
        setInput(e.target.value)
    }

    const newItemHandler = () => {
        updateStatus(input, "todo");
        reloadPage();
    }

    const doneItemHandler = (name) => {
        updateStatus(name, "done");
        reloadPage();
    }

    const deleteHandler = (name) => {
        deleteTask(name);
        reloadPage();
    }

    const renderItem = (item) => {
        if (item.status === 'todo') {
            return (
                <li className={styles.item} key={item.name}>
                    <div className={styles.itemName}>{item.name} : {item.status}</div>
                    <div className={styles.doneItemIcon} onClick={()=> doneItemHandler(item.name)}><DoneItem/></div>
                    <div className={styles.removeItemIcon} onClick={() => deleteHandler(item.name)}><RemoveItem/></div>
                </li>
            )
        } else {
            return (
                <li className={styles.doneItem} key={item.name}>
                    <div className={styles.itemName}>{item.name} : {item.status}</div>
                    <div className={styles.doneItemIcon} onClick={()=> doneItemHandler(item.name)}><DoneItem/></div>
                    <div className={styles.removeItemIcon} onClick={() => deleteHandler(item.name)}><RemoveItem/></div>
                </li>
            )
        }
    }

    useEffect(() => {
        setLoading(true)
        fetch('http://localhost:8080/todo/all')
            .then((res) => res.json())
            .then((data) => {
                setData(data)
                setLoading(false)
            })

    }, [])


    if (isLoading) return <p>Loading...</p>
    if (!data) return <p>No profile data</p>

    return (
        <div className={styles.todoContainer}>
            <h2>TASK LIST</h2>

            <div className={styles.newItemContainer}>
                <input
                    id={"newItemInput"}
                    className={styles.newTaskInput}
                    type={"text"}
                    placeholder={"Add new task"}
                    value={input}
                    onChange={inputHandler}
                />
                <div onClick={newItemHandler} className={styles.newItemIcon}><NewItem/></div>
            </div>

            <div>
                <ul>
                    {data.items.map(item => renderItem(item))}
                </ul>
            </div>
        </div>
    )
}
