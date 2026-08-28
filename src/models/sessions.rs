use chrono::NaiveDateTime;
use diesel::{Selectable, associations::Associations, deserialize::Queryable, prelude::Insertable};
use serde::Deserialize;
use uuid::Uuid;

use crate::{models::users::User, schema::sessions};

#[derive(Queryable, Selectable, Debug, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_id: Uuid,
}
