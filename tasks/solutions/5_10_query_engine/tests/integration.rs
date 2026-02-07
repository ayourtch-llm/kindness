use solution::*;

#[test]
fn select_all() {
    let mut db = Database::new();
    db.create_table("users", &["id", "name", "age"]);
    db.insert("users", vec!["1".into(), "Alice".into(), "30".into()]);
    db.insert("users", vec!["2".into(), "Bob".into(), "25".into()]);

    let rows = db.query("SELECT * FROM users").unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["1", "Alice", "30"]);
    assert_eq!(rows[1], vec!["2", "Bob", "25"]);
}

#[test]
fn select_columns() {
    let mut db = Database::new();
    db.create_table("users", &["id", "name", "age"]);
    db.insert("users", vec!["1".into(), "Alice".into(), "30".into()]);
    db.insert("users", vec!["2".into(), "Bob".into(), "25".into()]);

    let rows = db.query("SELECT name, age FROM users").unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["Alice", "30"]);
    assert_eq!(rows[1], vec!["Bob", "25"]);
}

#[test]
fn where_clause() {
    let mut db = Database::new();
    db.create_table("products", &["name", "category", "price"]);
    db.insert("products", vec!["Widget".into(), "A".into(), "10".into()]);
    db.insert("products", vec!["Gadget".into(), "B".into(), "20".into()]);
    db.insert("products", vec!["Doohickey".into(), "A".into(), "15".into()]);

    let rows = db.query("SELECT name, price FROM products WHERE category = 'A'").unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["Widget", "10"]);
    assert_eq!(rows[1], vec!["Doohickey", "15"]);
}

#[test]
fn order_by() {
    let mut db = Database::new();
    db.create_table("items", &["name", "value"]);
    db.insert("items", vec!["Banana".into(), "2".into()]);
    db.insert("items", vec!["Apple".into(), "1".into()]);
    db.insert("items", vec!["Cherry".into(), "3".into()]);

    let rows = db.query("SELECT name FROM items ORDER BY name").unwrap();
    assert_eq!(rows, vec![vec!["Apple"], vec!["Banana"], vec!["Cherry"]]);

    let rows_desc = db.query("SELECT name FROM items ORDER BY name DESC").unwrap();
    assert_eq!(rows_desc, vec![vec!["Cherry"], vec!["Banana"], vec!["Apple"]]);
}

#[test]
fn unknown_table_error() {
    let db = Database::new();
    let result = db.query("SELECT * FROM nonexistent");
    assert!(result.is_err());
}

#[test]
fn where_and_order_combined() {
    let mut db = Database::new();
    db.create_table("scores", &["name", "team", "points"]);
    db.insert("scores", vec!["Zara".into(), "red".into(), "50".into()]);
    db.insert("scores", vec!["Amy".into(), "blue".into(), "70".into()]);
    db.insert("scores", vec!["Mike".into(), "red".into(), "30".into()]);
    db.insert("scores", vec!["Jake".into(), "red".into(), "90".into()]);

    let rows = db.query("SELECT name, points FROM scores WHERE team = 'red' ORDER BY name").unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0], vec!["Jake", "90"]);
    assert_eq!(rows[1], vec!["Mike", "30"]);
    assert_eq!(rows[2], vec!["Zara", "50"]);
}
