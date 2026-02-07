use std::collections::HashMap;

pub struct Database {
    tables: HashMap<String, Table>,
}

struct Table {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: &str, columns: &[&str]) {
        self.tables.insert(
            name.to_string(),
            Table {
                columns: columns.iter().map(|s| s.to_string()).collect(),
                rows: Vec::new(),
            },
        );
    }

    pub fn insert(&mut self, table: &str, row: Vec<String>) {
        if let Some(t) = self.tables.get_mut(table) {
            t.rows.push(row);
        }
    }

    pub fn query(&self, sql: &str) -> Result<Vec<Vec<String>>, String> {
        let tokens = tokenize(sql);
        let parsed = parse_query(&tokens)?;
        self.execute(parsed)
    }

    fn execute(&self, q: Query) -> Result<Vec<Vec<String>>, String> {
        let table = self
            .tables
            .get(&q.table)
            .ok_or_else(|| format!("Unknown table: {}", q.table))?;

        // Resolve selected column indices
        let select_indices: Vec<usize> = if q.select_all {
            (0..table.columns.len()).collect()
        } else {
            q.columns
                .iter()
                .map(|c| {
                    table
                        .columns
                        .iter()
                        .position(|tc| tc == c)
                        .ok_or_else(|| format!("Unknown column: {}", c))
                })
                .collect::<Result<Vec<_>, _>>()?
        };

        // Filter rows
        let mut rows: Vec<&Vec<String>> = table.rows.iter().collect();

        if let Some(ref wc) = q.where_clause {
            let col_idx = table
                .columns
                .iter()
                .position(|c| c == &wc.column)
                .ok_or_else(|| format!("Unknown column: {}", wc.column))?;
            rows.retain(|row| row[col_idx] == wc.value);
        }

        // Order by
        if let Some(ref ob) = q.order_by {
            let col_idx = table
                .columns
                .iter()
                .position(|c| c == &ob.column)
                .ok_or_else(|| format!("Unknown column: {}", ob.column))?;
            rows.sort_by(|a, b| {
                let cmp = a[col_idx].cmp(&b[col_idx]);
                if ob.desc {
                    cmp.reverse()
                } else {
                    cmp
                }
            });
        }

        // Project
        let result: Vec<Vec<String>> = rows
            .iter()
            .map(|row| select_indices.iter().map(|&i| row[i].clone()).collect())
            .collect();

        Ok(result)
    }
}

struct Query {
    select_all: bool,
    columns: Vec<String>,
    table: String,
    where_clause: Option<WhereClause>,
    order_by: Option<OrderBy>,
}

struct WhereClause {
    column: String,
    value: String,
}

struct OrderBy {
    column: String,
    desc: bool,
}

fn tokenize(sql: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = sql.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        if chars[i] == '\'' {
            // String literal
            i += 1;
            let start = i;
            while i < chars.len() && chars[i] != '\'' {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            tokens.push(format!("'{}'", s));
            if i < chars.len() {
                i += 1; // skip closing quote
            }
            continue;
        }

        if chars[i] == ',' || chars[i] == '=' {
            tokens.push(chars[i].to_string());
            i += 1;
            continue;
        }

        if chars[i] == '*' {
            tokens.push("*".to_string());
            i += 1;
            continue;
        }

        // Word token
        let start = i;
        while i < chars.len() && !chars[i].is_whitespace() && chars[i] != ',' && chars[i] != '=' {
            i += 1;
        }
        let word: String = chars[start..i].iter().collect();
        tokens.push(word);
    }

    tokens
}

fn parse_query(tokens: &[String]) -> Result<Query, String> {
    let mut i = 0;

    // Expect SELECT
    if i >= tokens.len() || tokens[i].to_uppercase() != "SELECT" {
        return Err("Expected SELECT".into());
    }
    i += 1;

    // Parse columns
    let mut select_all = false;
    let mut columns = Vec::new();

    if i < tokens.len() && tokens[i] == "*" {
        select_all = true;
        i += 1;
    } else {
        loop {
            if i >= tokens.len() {
                return Err("Expected column name".into());
            }
            columns.push(tokens[i].clone());
            i += 1;
            if i < tokens.len() && tokens[i] == "," {
                i += 1; // skip comma
            } else {
                break;
            }
        }
    }

    // Expect FROM
    if i >= tokens.len() || tokens[i].to_uppercase() != "FROM" {
        return Err("Expected FROM".into());
    }
    i += 1;

    // Table name
    if i >= tokens.len() {
        return Err("Expected table name".into());
    }
    let table = tokens[i].clone();
    i += 1;

    // Optional WHERE
    let mut where_clause = None;
    if i < tokens.len() && tokens[i].to_uppercase() == "WHERE" {
        i += 1;
        if i + 2 >= tokens.len() {
            return Err("Incomplete WHERE clause".into());
        }
        let col = tokens[i].clone();
        i += 1;
        if tokens[i] != "=" {
            return Err("Expected = in WHERE clause".into());
        }
        i += 1;
        let val_token = &tokens[i];
        let value = if val_token.starts_with('\'') && val_token.ends_with('\'') {
            val_token[1..val_token.len() - 1].to_string()
        } else {
            val_token.clone()
        };
        i += 1;
        where_clause = Some(WhereClause { column: col, value });
    }

    // Optional ORDER BY
    let mut order_by = None;
    if i < tokens.len() && tokens[i].to_uppercase() == "ORDER" {
        i += 1;
        if i >= tokens.len() || tokens[i].to_uppercase() != "BY" {
            return Err("Expected BY after ORDER".into());
        }
        i += 1;
        if i >= tokens.len() {
            return Err("Expected column in ORDER BY".into());
        }
        let col = tokens[i].clone();
        i += 1;
        let desc = if i < tokens.len() && tokens[i].to_uppercase() == "DESC" {
            i += 1;
            true
        } else {
            false
        };
        order_by = Some(OrderBy { column: col, desc });
    }

    Ok(Query {
        select_all,
        columns,
        table,
        where_clause,
        order_by,
    })
}
