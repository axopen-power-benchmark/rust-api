# Rust API avec Rocket et Diesel


## Dépendances

Pour compiler le projet les dépendances suivantes doivent être installé :
```
rustup # qui installera
# cargo
# rustc
# rust

mysqlclient_lib # la variable d'environnement MYSQLCLIENT_LIB_DIR doit pointer dessus
```

## Compilation

```shell
# Build
cd ./api
cargo build --release

# Démarrage du serveur
./target/release/rust_api
```

## Configuration

Dans dossier courant lors du lancement de l'API doit se trouver un fichier 
```Rocket.toml``` au même format que celui se trouvant dans ```./api/Rocket.toml```

On retrouve la configuration de la base de données dans ce fichier :
```toml
[global]
address = "0.0.0.0"
port = 8001

[global.databases]
test_db = { url = "mysql://axopen:motdepasse@localhost:3306/test_perf", pool_size = 20 }
```

## Route

### GET /api/chantier

Retourne un chantier random en mode eager

### POST /api/chantier

Update un chantier random avec des valeurs random et retourne le chantier updater
