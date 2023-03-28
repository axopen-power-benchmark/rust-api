table! {
    chef_chantier_user (numero_chantier, matricule_user) {
        numero_chantier -> Integer,
        matricule_user -> Integer,
    }
}

table! {
    chantier (numero) {
        numero -> Integer,
        description -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        city_cp -> Nullable<Integer>,
        date_debut -> Nullable<Date>,
        date_fin -> Nullable<Date>,
        status -> Nullable<Varchar>,
        lien_sharepoint -> Nullable<Varchar>,
        lien_files -> Nullable<Varchar>,
        lien_gearth -> Nullable<Varchar>,
        prix_moyen_moe_jour -> Nullable<Integer>,
        prix_moyen_moe_nuit -> Nullable<Integer>,
        prix_moyen_materiel -> Nullable<Integer>,
        journal_pointage_erp -> Nullable<Varchar>,
    }
}

table! {
    canopee_user (matricule) {
        matricule -> Nullable<Integer>,
        fullname -> Nullable<Varchar>,
        societe -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        job_title -> Nullable<Varchar>,
        resource_group_no -> Nullable<Varchar>,
        travel_code -> Nullable<Varchar>,
        numero_latest_chantier -> Nullable<Integer>,
        journal_pointage_erp -> Nullable<Varchar>,
    }
}

table! {
    depense_main_doeuvre (id) {
        id -> Bigint,
        id_journal_chantier -> Nullable<Integer>,
        id_ouvrier_chantier -> Nullable<Integer>,
        quart_dheures_jour -> Nullable<Smallint>,
        quart_dheures_nuit -> Nullable<Smallint>,
        voiture -> Nullable<Bool>,
        sent -> Nullable<Bool>,
    }
}

table! {
    journal_chantier (id) {
        id -> Integer,
        numero_chantier -> Integer,
        date -> Nullable<Date>,
        temperature_matin -> Nullable<Smallint>,
        temperature_soir -> Nullable<Smallint>,
        moe_generated -> Nullable<Bool>,
        materiel_generated -> Nullable<Bool>,
        note -> Nullable<Varchar>,
    }
}

table! {
    ouvrier_chantier (id) {
        id -> Integer,
        numero_chantier -> Integer,
        matricule_user -> Nullable<Integer>,
        matricule -> Nullable<Varchar>,
        unbind -> Nullable<Bool>,
    }
}

joinable!(canopee_user -> chantier (numero_latest_chantier));
joinable!(chef_chantier_user -> canopee_user (matricule_user));
joinable!(chef_chantier_user -> chantier (numero_chantier));
joinable!(depense_main_doeuvre -> journal_chantier (id_journal_chantier));
joinable!(depense_main_doeuvre -> ouvrier_chantier (id_ouvrier_chantier));
joinable!(journal_chantier -> chantier (numero_chantier));
joinable!(ouvrier_chantier -> canopee_user (matricule_user));
joinable!(ouvrier_chantier -> chantier (numero_chantier));


allow_tables_to_appear_in_same_query!(
    canopee_user,
    chantier,
    chef_chantier_user,
    depense_main_doeuvre,
    journal_chantier,
    ouvrier_chantier,
);
