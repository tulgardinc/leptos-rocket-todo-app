#!/bin/bash

export DATABASE_URL="sqlite:./backend/todos.db"
export DIST_DIR="./backend/dist"


DATABASE="./backend/todos.db"
if [ ! -f "$DATABASE" ]; then
    echo "Database does not exist. Creating database and table..."
    sqlite3 $DATABASE <<EOF
CREATE TABLE todos (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    is_complete BOOL NOT NULL
);
EOF
fi

cargo run --package backend