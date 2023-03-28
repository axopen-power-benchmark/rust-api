use rocket::serde::Serialize;
use diesel::{self, result::QueryResult, prelude::*};

use crate::schema::{chantier};

use chrono::naive::{NaiveDate};

use crate::{ChefChantierUser, DbConn};
use crate::canopee_user_model::CanopeeUser;
use crate::depense_main_doeuvre_model::DepenseMainDoeuvre;
use crate::journal_chantier_model::{JournalChantier, JournalChantierEager};
use crate::ouvrier_chantier_model::OuvrierChantier;

#[derive(Serialize, Identifiable, Queryable, Insertable, AsChangeset, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = chantier)]
#[diesel(primary_key(numero))]
pub struct Chantier {
    #[serde(skip_deserializing)]
    pub numero: i32,
    pub description: Option<String>,
    pub city: Option<String>,
    pub city_cp: Option<i32>,
    pub date_debut: Option<NaiveDate>,
    pub date_fin: Option<NaiveDate>,
    pub status: Option<String>,
    pub lien_sharepoint: Option<String>,
    pub lien_files: Option<String>,
    pub lien_gearth: Option<String>,
    pub prix_moyen_moe_jour: Option<i32>,
    pub prix_moyen_moe_nuit: Option<i32>,
    pub prix_moyen_materiel: Option<i32>,
    pub journal_pointage_erp: Option<String>,
}


#[derive(Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ChantierEager {
    pub numero: i32,
    pub description: Option<String>,
    pub city: Option<String>,
    pub city_cp: Option<i32>,
    pub date_debut: Option<NaiveDate>,
    pub date_fin: Option<NaiveDate>,
    pub status: Option<String>,
    pub lien_sharepoint: Option<String>,
    pub lien_files: Option<String>,
    pub lien_gearth: Option<String>,
    pub prix_moyen_moe_jour: Option<i32>,
    pub prix_moyen_moe_nuit: Option<i32>,
    pub prix_moyen_materiel: Option<i32>,
    pub journal_pointage_erp: Option<String>,
    pub ouvrier_chantier: Vec<OuvrierChantier>,
    pub chef_chantier: Vec<ChefChantierUser>,
    pub canopee_user: Vec<CanopeeUser>,
    pub journal_chantier: Vec<JournalChantierEager>,
}

impl Chantier {

    pub async fn one(conn: &DbConn, chantier_numero: i32) -> QueryResult<Chantier> {
        conn.run(move |c| {
            chantier::table
                .filter(chantier::numero.eq(chantier_numero))
                .first
            ::<Chantier>(c)
        }).await
    }

    pub async fn one_eager(conn: &DbConn, chantier_numero: i32) -> Result<Option<ChantierEager>, Option<ChantierEager>> {
        conn.run(move |c| {
            let chantier = chantier::table
                .filter(chantier::numero.eq(chantier_numero))
                .first::<Chantier>(c).expect("Error");
            let chef_chantier = ChefChantierUser::belonging_to(&chantier)
                .load::<ChefChantierUser>(c).expect("Error");
            let ouvrier_chantier = OuvrierChantier::belonging_to(&chantier)
                .load::<OuvrierChantier>(c).expect("Error");
            let canopee_user = CanopeeUser::belonging_to(&chantier)
                .load::<CanopeeUser>(c).expect("Error");
            let journal_chantier: Vec<JournalChantier> = JournalChantier::belonging_to(&chantier)
                .load::<JournalChantier>(c).expect("Error");
            let depense_main_doeuvre = DepenseMainDoeuvre::belonging_to(&journal_chantier)
                .load::<DepenseMainDoeuvre>(c).expect("Moire");

            let grouped_depense: Vec<Vec<DepenseMainDoeuvre>> = depense_main_doeuvre.grouped_by(&journal_chantier);
            let journal_and_depense_vec: Vec<(JournalChantier, Vec<DepenseMainDoeuvre>)> = journal_chantier
                .into_iter()
                .zip(grouped_depense)
                .collect();

            let journal_and_depense = journal_and_depense_vec
                .iter()
                .map(|j| JournalChantierEager::from_db_result(j.clone()))
                .collect();

            Ok(Some(ChantierEager {
                numero: chantier.numero,
                description: chantier.description,
                city: chantier.city,
                city_cp: chantier.city_cp,
                date_debut: chantier.date_debut,
                date_fin: chantier.date_fin,
                status: chantier.status,
                lien_sharepoint: chantier.lien_sharepoint,
                lien_files: chantier.lien_files,
                lien_gearth: chantier.lien_gearth,
                prix_moyen_moe_jour: chantier.prix_moyen_moe_jour,
                prix_moyen_moe_nuit: chantier.prix_moyen_moe_nuit,
                prix_moyen_materiel: chantier.prix_moyen_materiel,
                chef_chantier,
                ouvrier_chantier,
                canopee_user,
                journal_chantier: journal_and_depense,
                journal_pointage_erp: None
            }))
        }).await
    }

    /// Returns the number of affected rows: 1.
    pub async fn update(conn: &DbConn, chantier_to_update: Chantier) -> QueryResult<Chantier> {
        conn.run(move |c| {
            let chantier_updated = chantier_to_update.clone();
            diesel::update(chantier::table).set(chantier_updated)
                .filter(chantier::numero.eq(chantier_to_update.numero))
                .execute(c)
                .expect("Panic on update chantier");
            chantier::table
                .filter(chantier::numero.eq(chantier_to_update.numero.clone()))
                .first
                    ::<Chantier>(c)
        }).await
    }
}
