use rocket::serde::Serialize;
use diesel::{self, prelude::*};

use crate::schema::ouvrier_chantier;
use crate::{Chantier};


#[derive(Serialize, Identifiable, Associations, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Chantier, foreign_key = numero_chantier))]
#[diesel(table_name = ouvrier_chantier)]
pub struct OuvrierChantier {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub numero_chantier: i32,
    pub matricule_user: Option<i32>,
    pub matricule: Option<String>,
    pub unbind: Option<bool>,
}
