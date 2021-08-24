# Queue

Projet du cours de processus stochastiques.

## Installation & utilisation

### Rust
Pour compiler ce projet, il est nécessaire d'installer rust.

La commande suivante devrait suffire pour Linux & Mac:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Sur windows il est possible de passer par des gestionnaires de paquets comme 
[Chocolatey](https://chocolatey.org/install) et d'utiliser la commande `choco install rust`.

### Compiler et lancer le projet

Une fois que rust est installé, il faut se placer dans le dossier contenant le fichier `Cargo.toml` et exécuter la commande suivante:
```shell
cargo run
```

Cette commande va `build` et `run` le projet, c'est à dire récupérer toutes les dépendances, compiler tout et lancer l'exécutable généré.

## Résultats

### Exponentielle

![exp_avg](images/exp_avg_stay_by_rho.png)
![exp_p_off](images/exp_p_off_by_rho.png)
![exp_p_setup](images/exp_p_setup_by_rho.png)

### Erlang

![erlang_avg](images/erlang_avg_stay_by_rho.png)
![erlang_p_off](images/erlang_p_off_by_rho.png)
![erlang_p_setup](images/erlang_p_setup_by_rho.png)

### E[W²]

25.65760339747278

## Auteur

Guillaume Latour < guillaume dot latour at student dot unamur dot be >