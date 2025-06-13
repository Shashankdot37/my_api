// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Int4,
        pname -> Varchar,
        jersey_no -> Int4,
        available -> Bool,
    }
}

//can use infer_schema! when needed to infer the database based on the schema.