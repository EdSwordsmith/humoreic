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
        embed_ids -> Jsonb,
        msg_ids -> Jsonb,
    }
}

table! {
    reactions (id) {
        id -> Int8,
        reaction -> Varchar,
        message_id -> Int8,
        user_id -> Int8,
    }
}

joinable!(reactions -> messages (message_id));

allow_tables_to_appear_in_same_query!(
    admins,
    bans,
    guilds,
    messages,
    reactions,
);
