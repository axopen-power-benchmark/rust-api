#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

mod schema;

#[cfg(test)]
mod tests;
mod canopee_user_model;
mod chantier_model;
mod chef_chantier_user_model;
mod journal_chantier_model;
mod depense_main_doeuvre_model;
mod ouvrier_chantier_model;
mod utils;

use chrono::NaiveDate;

use rocket::request::FlashMessage;
use rocket::serde::Serialize;
use rocket::serde::json::Json;

use crate::chantier_model::{Chantier, ChantierEager};
use crate::chef_chantier_user_model::ChefChantierUser;
use crate::journal_chantier_model::JournalChantier;

#[database("test_db")]
pub struct DbConn(diesel::MysqlConnection);

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct ContextFind {
    flash: Option<(String, String)>,
    tasks: Option<ChantierEager>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct ContextUpdate {
    flash: Option<(String, String)>,
    tasks: Option<Chantier>,
}

impl ContextFind {

    pub async fn find_chantier(conn: &DbConn, flash: Option<(String, String)>) -> ContextFind {
        let random_chantier = utils::random_int();

        match Chantier::one_eager(conn, random_chantier).await {
            Ok(data) => {
                ContextFind {
                    flash,
                    tasks: data
                }
            },
            Err(e) => {
                ContextFind {
                    flash,
                    tasks: e,
                }
            }
        }
    }
}

impl ContextUpdate {

    pub async fn update_chantier(conn: &DbConn, flash: Option<(String, String)>) -> ContextUpdate {
        let random_chantier = utils::random_int();

        let chantier = Chantier {
            numero: random_chantier,
            description: Some(utils::random_string()),
            city: Some(utils::random_string()),
            city_cp: Some(utils::random_int()),
            date_debut: NaiveDate::from_ymd_opt(
                utils::random_int_range(2000, 2030),
                utils::random_uint_range(1, 12),
                utils::random_uint_range(1, 28)),
            date_fin: NaiveDate::from_ymd_opt(
                utils::random_int_range(2000, 2030),
                utils::random_uint_range(1, 12),
                utils::random_uint_range(1, 28)),
            status: Some(utils::random_string()),
            lien_sharepoint: Some(utils::random_string()),
            lien_files: Some(utils::random_string()),
            lien_gearth: Some(utils::random_string()),
            prix_moyen_moe_jour: Some(utils::random_int()),
            prix_moyen_moe_nuit: Some(utils::random_int()),
            prix_moyen_materiel: Some(utils::random_int()),
            journal_pointage_erp: Some(utils::random_string()),
        };

        match Chantier::update(conn, chantier).await {
            Ok(data) => {
                ContextUpdate {
                    flash,
                    tasks: Some(data)
                }
            },
            Err(_e) => {
                ContextUpdate {
                    flash,
                    tasks: None,
                }
            }
        }
    }
}

#[get("/")]
async fn get_chantier(flash: Option<FlashMessage<'_>>, conn: DbConn) -> Json<ChantierEager> {
    let flash = flash.map(FlashMessage::into_inner);
    let result = ContextFind::find_chantier(&conn, flash).await;
    Json(result.tasks.unwrap())
}

#[post("/")]
async fn post_chantier(flash: Option<FlashMessage<'_>>, conn: DbConn) -> Json<Chantier> {
    let flash = flash.map(FlashMessage::into_inner);
    let result = ContextUpdate::update_chantier(&conn, flash).await;
    Json(result.tasks.unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/api/chantier", routes![get_chantier, post_chantier])
}
