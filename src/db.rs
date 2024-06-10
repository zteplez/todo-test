use clap::Error;
use rusqlite::{params, Connection, Result, Rows};

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub completed: i32,
}

pub fn connect() -> Result<Connection> {
    let conn = Connection::open("C:/Users/zteplez/Desktop/Projects/todo-app/main.db")?;
    Ok(conn)
}
pub fn insert_data(conn: &Connection, name: &str, completed: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks(name, completed) VALUES(?1, ?2)",
        params![name, completed],
    )?;
    Ok(())
}
pub fn show_data(conn: &Connection) -> Result<Vec<Task>> {
    let mut statement = conn.prepare("SELECT id, name, completed FROM tasks")?;
    let task_iteration = statement.query_map([], |row| {
        Ok(Task {
            id: row
                .get::<_, i32>(0)?
                .try_into()
                .expect("Convertion failed."),
            name: row.get(1)?,
            completed: row
                .get::<_, i32>(2)?
                .try_into()
                .expect("Convertion failed."),
        })
    })?;
    let mut tasks = Vec::new();
    for task in task_iteration {
        tasks.push(task?);
    }
    Ok(tasks)
}
pub fn delete_data(conn: &Connection, id: i32) -> Result<()> {
    //let id = id.to_string();
    println!("Task {} deleted.", id);
    conn.execute("DELETE FROM tasks WHERE id=?1", params![id])?;
    Ok(())
}
