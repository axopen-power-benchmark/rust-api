use rocket::serde::Serialize;
use diesel::{self, prelude::*};

use crate::schema::chef_chantier_user;
use crate::{Chantier};

#[derive(Serialize, Identifiable, Associations, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(primary_key(numero_chantier, matricule_user))]
#[diesel(belongs_to(Chantier, foreign_key = numero_chantier))]
#[diesel(table_name = chef_chantier_user)]
pub struct ChefChantierUser {
    #[serde(skip_deserializing)]
    pub numero_chantier: i32,
    pub matricule_user: i32,
}
