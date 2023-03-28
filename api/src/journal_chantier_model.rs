use chrono::NaiveDate;
use rocket::serde::Serialize;
use diesel::{self, prelude::*};

use crate::schema::journal_chantier;
use crate::{Chantier};
use crate::depense_main_doeuvre_model::DepenseMainDoeuvre;

#[derive(Serialize, Identifiable, Associations, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Chantier, foreign_key = numero_chantier))]
#[diesel(table_name = journal_chantier)]
pub struct JournalChantier {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub numero_chantier: i32,
    pub date: Option<NaiveDate>,
    pub temperature_matin: Option<i16>,
    pub temperature_soir: Option<i16>,
    pub moe_generated: Option<bool>,
    pub materiel_generated: Option<bool>,
    pub note: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct JournalChantierEager {
    pub id: i32,
    pub numero_chantier: i32,
    pub date: Option<NaiveDate>,
    pub temperature_matin: Option<i16>,
    pub temperature_soir: Option<i16>,
    pub moe_generated: Option<bool>,
    pub materiel_generated: Option<bool>,
    pub note: Option<String>,
    pub depense_main_doeuvre: Vec<DepenseMainDoeuvre>,
}

impl JournalChantierEager {
    pub fn from_db_result(journal_with_depenses: (JournalChantier, Vec<DepenseMainDoeuvre>)) -> JournalChantierEager {
        let j2 = journal_with_depenses.clone();
        JournalChantierEager {
            id: j2.0.id,
            date: j2.0.date,
            numero_chantier:j2.0.numero_chantier,
            temperature_matin:j2.0.temperature_matin,
            temperature_soir:j2.0.temperature_soir,
            moe_generated:j2.0.moe_generated,
            materiel_generated:j2.0.materiel_generated,
            note:j2.0.note,
            depense_main_doeuvre:j2.1,
        }
    }
}
