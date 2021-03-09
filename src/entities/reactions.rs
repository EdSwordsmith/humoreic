use crate::schema::{messages, reactions};
use diesel::sql_types::{BigInt, VarChar};
use diesel::*;
use std::collections::HashMap;

#[derive(Queryable, QueryableByName, Clone)]
pub struct SavedReaction {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "VarChar"]
    pub reaction: String,
    #[sql_type = "BigInt"]
    pub message_id: i64,
    #[sql_type = "BigInt"]
    pub channel_id: i64,
    #[sql_type = "BigInt"]
    pub user_id: i64,
}

#[derive(Insertable)]
#[table_name = "reactions"]
pub struct NewReaction {
    pub reaction: String,
    pub message_id: i64,
    pub channel_id: i64,
    pub user_id: i64,
}

pub fn create_reaction(
    conn: &PgConnection,
    message_id: i64,
    reaction: &String,
    user_id: i64,
    channel_id: i64,
) -> SavedReaction {
    let new_reaction = NewReaction {
        message_id,
        reaction: (*reaction).clone(),
        user_id,
        channel_id,
    };

    diesel::insert_into(reactions::table)
        .values(new_reaction)
        .get_result(conn)
        .expect(&format!("Couldn't insert reaction in message {} in table", message_id))
}

pub fn delete_reaction(conn: &PgConnection, message: i64, r: &String, user: i64) {
    use crate::schema::reactions::dsl::*;

    diesel::delete(
        reactions
            .filter(reaction.eq(r))
            .filter(user_id.eq(user))
            .filter(message_id.eq(message)),
    )
    .execute(conn)
    .expect(&format!("Couldn't delete reaction in message {} in table", message));
}

pub fn get_reactions(conn: &PgConnection, message_id: i64) -> HashMap<String, Vec<SavedReaction>> {
    let reactions: Vec<SavedReaction> = reactions::table
        .inner_join(
            messages::table.on(reactions::message_id
                .eq(messages::id)
                .and(reactions::message_id.eq(message_id))),
        )
        .select((
            reactions::id,
            reactions::reaction,
            reactions::message_id,
            reactions::channel_id,
            reactions::user_id,
        ))
        .load(conn)
        .expect("Couldn't get reactions");

    let mut reactions_group = HashMap::<String, Vec<SavedReaction>>::new();
    for r in reactions.iter() {
        if !reactions_group.contains_key(&r.reaction) {
            reactions_group.insert(r.reaction.clone(), Vec::new());
        }

        reactions_group
            .get_mut(&r.reaction)
            .expect("Couldn't get mutex")
            .push((*r).clone());
    }

    return reactions_group;
}

pub fn has_reaction(
    reactions: &HashMap<String, Vec<SavedReaction>>,
    reaction: &String,
    user_id: i64,
) -> bool {
    let reactions: Option<&Vec<SavedReaction>> = reactions.get(reaction);
    if let Some(rs) = reactions {
        for r in rs.iter() {
            if r.user_id == user_id {
                return true;
            }
        }
    }

    false
}

pub fn reaction_actually_exists(
    reactions: &HashMap<String, Vec<SavedReaction>>,
    reaction: &String,
    user_id: i64,
    channel_id: i64,
) -> bool {
    let reactions: Option<&Vec<SavedReaction>> = reactions.get(reaction);
    if let Some(rs) = reactions {
        for r in rs.iter() {
            if r.user_id == user_id && r.channel_id == channel_id {
                return true;
            }
        }
    }

    false
}
