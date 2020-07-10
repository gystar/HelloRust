use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const SPLITTER: char = '|'; //字段之间的分割符号
const MAX_ROWS: usize = 256;
struct Address {
    id: usize,
    set: i32,
    name: String,
    email: String,
}
struct Database {
    rows: Vec<Address>,
}

struct Connection {
    file: Option<File>,
    db: Database,
}

fn address_print(addr: &Address) {
    print!("{:?} {:?} {:?}\n", addr.id, addr.name, addr.email);
}

fn database_load(conn: &mut Connection) {
    let mut buff = String::new();
    match conn.file.as_ref().unwrap().read_to_string(&mut buff) {
        Ok(_) => {}
        Err(_) => panic!("Failed to load database."),
    }
    let mut index = 0;
    for line in buff.lines() {
        let v = line.split(SPLITTER).collect::<Vec<_>>();
        conn.db.rows[index].id = v[0].parse::<usize>().unwrap();
        conn.db.rows[index].set = v[1].parse::<i32>().unwrap();
        conn.db.rows[index].name = v[2].to_string();
        conn.db.rows[index].email = v[3].to_string();
        index += 1;
    }
}

fn database_open(filename: &String, mode: char) -> Connection {
    let mut con = Connection {
        file: None,
        db: Database {
            rows: Vec::<Address>::new(),
        },
    };
    for _ in 0..MAX_ROWS {
        con.db.rows.push(Address {
            id: MAX_ROWS,
            set: 0,
            name: String::new(),
            email: String::new(),
        });
    }
    if mode == 'c' {
        match OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(filename)
        {
            Ok(f) => {
                con.file = Some(f);
            }
            Err(_) => {
                panic!("fail to create \"{:?}\"", filename);
            }
        }
    } else {
        match OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(filename)
        {
            Ok(f) => {
                con.file = Some(f);
                database_load(&mut con);
            }
            Err(_) => {
                panic!("fail to open \"{:?}\"", filename);
            }
        }
    }
    con
}

fn database_create(conn: &mut Connection) {
    let mut index = 0;
    for r in conn.db.rows.iter_mut() {
        r.id = index;
        r.set = 0;
        index += 1;
    }
}

fn database_write(conn: &Connection) {
    let mut data = String::new();
    let mut row = 1;
    for addr in conn.db.rows.iter() {
        data += &addr.id.to_string();
        data.push(SPLITTER);
        data += &addr.set.to_string();
        data.push(SPLITTER);
        data += &addr.name;
        data.push(SPLITTER);
        data += &addr.email;
        if row < MAX_ROWS {
            data.push('\n');
        }
        row += 1;
    }

    let _ = conn.file.as_ref().unwrap().set_len(0); //先清空文件内容
    match conn.file.as_ref().unwrap().write(data.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            panic!("Failed to write database.");
        }
    }
}

fn database_set(conn: &mut Connection, index: usize, name: &str, email: &str) {
    let addr = &mut conn.db.rows[index];
    if addr.set == 1 {
        panic!("Already set, delete it first");
    }

    addr.set = 1;
    // WARNING: bug, read the "How To Break It" and fix this
    addr.name = name.to_string();
    addr.email = email.to_string();
}

fn database_get(conn: &Connection, id: usize) {
    let addr: &Address = &conn.db.rows[id];
    if addr.set == 1 {
        address_print(addr);
    } else {
        panic!("ID is not set");
    }
}

fn database_delete(conn: &mut Connection, id: usize) {
    let addr: &mut Address = &mut conn.db.rows[id];
    addr.set = 0;
}

fn database_list(conn: &Connection) {
    for r in conn.db.rows.iter() {
        address_print(r);
        if r.set == 1 {
            //address_print(r);
        }
    }
}

fn main() {
    let mut args = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }
    if args.len() < 3 {
        panic!("USAGE: ex17 <dbfile> <action> [action params]");
    }

    let mut filepath = env::current_dir().unwrap().to_str().unwrap().to_string();
    println!("{:?}", filepath);
    filepath += "\\";
    filepath += &args[1];
    let action = args[2].chars().next().unwrap();

    let mut conn = database_open(&filepath, action);
    let mut id = 0;

    if args.len() > 3 {
        id = args[3].parse::<usize>().unwrap();
    }
    if id >= MAX_ROWS {
        panic!("There's not that many records.");
    }

    if action == 'c' {
        database_create(&mut conn);
        database_write(&mut conn);
    } else if action == 'g' {
        if args.len() < 4 {
            panic!("Need an id to get");
        }
        database_get(&conn, id);
    } else if action == 's' {
        if args.len() < 6 {
            panic!("Need id, name, email to set");
        }
        database_set(&mut conn, id, &args[4], &args[5]);
        database_list(&conn);
        database_write(&mut conn);
    } else if action == 'd' {
        if args.len() < 4 {
            panic!("Need an id to get");
        }

        database_delete(&mut conn, id);
        database_write(&mut conn);
    } else if action == 'l' {
        database_list(&conn);
    } else {
        panic!("Invalid action, only: c=create, g=get, s=set, d=del, l=list");
    }
}
