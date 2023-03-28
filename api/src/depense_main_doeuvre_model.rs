use rocket::serde::Serialize;
use diesel::{self, prelude::*};

use crate::schema::{depense_main_doeuvre};
use crate::{JournalChantier};

#[derive(Serialize, Identifiable, Associations, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(primary_key(id))]
#[diesel(belongs_to(JournalChantier, foreign_key = id_journal_chantier))]
#[diesel(table_name = depense_main_doeuvre)]
pub struct DepenseMainDoeuvre {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub id_journal_chantier: Option<i32>,
    pub id_ouvrier_chantier: Option<i32>,
    pub quart_dheures_jour: Option<i16>,
    pub quart_dheures_nuit: Option<i16>,
    pub voiture: Option<bool>,
    pub sent: Option<bool>,
}
