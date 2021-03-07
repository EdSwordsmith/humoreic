table! {
    admins (id) {
        id -> Int8,
    }
}

table! {
    bans (id) {
        id -> Int8,
    }
}

table! {
    guilds (id) {
        id -> Int8,
        channel_id -> Int8,
    }
}

table! {
    messages (id) {
        id -> Int8,
        others -> Jsonb,
        reactions -> Jsonb,
    }
}

allow_tables_to_appear_in_same_query!(admins, bans, guilds, messages,);
