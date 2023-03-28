use rocket::serde::Serialize;
use diesel::{self, prelude::*};

use crate::schema::canopee_user;
use crate::{Chantier};


#[derive(Serialize, Identifiable, Associations, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(primary_key(matricule))]
#[diesel(belongs_to(Chantier, foreign_key = numero_latest_chantier))]
#[diesel(table_name = canopee_user)]
pub struct CanopeeUser {
    #[serde(skip_deserializing)]
    pub matricule: Option<i32>,
    pub fullname: Option<String>,
    pub societe: Option<String>,
    pub email: Option<String>,
    pub job_title: Option<String>,
    pub resource_group_no: Option<String>,
    pub travel_code: Option<String>,
    pub numero_latest_chantier: Option<i32>,
    pub journal_pointage_erp: Option<String>,
}
