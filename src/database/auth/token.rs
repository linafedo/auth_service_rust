use crate::database::auth::models::{CreateToken, User};
use crate::schema::tokens::dsl::*;
use crate::schema::tokens;
use diesel::{RunQueryDsl, Insertable};
use rand_core::RngCore;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = tokens)]
pub struct NewToken<'a>  {
    pub token: &'a str,
    pub user_id: &'a Uuid
}

fn create_token(db: &mut diesel::PgConnection, user: &User) -> CreateToken {
    let mut token_bytes = [0u8, 32];
    rand_core::OsRng.fill_bytes(&mut token_bytes);
    let token_string = base64::encode(token_bytes);
    let new_token = NewToken{
        token: &token_string,
        user_id: &user.id
    };

    match diesel::insert_into(tokens)
        .values(new_token)
        .execute(db) {
        Ok(_) => CreateToken::Ok(token_string),
        Err(e) => {
            eprintln!("Error inserting token: {}", e);
            CreateToken::Error
        }
    }
}




