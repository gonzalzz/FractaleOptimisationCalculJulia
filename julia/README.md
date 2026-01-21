# Visualisation de l'ensemble de Julia

Ce projet est un moteur de rendu de fractales (Ensemble de Julia) écrit en Rust. Il permet d'explorer l'ensemble de manière interactive en ajustant les paramètres et la vue en temps réel.
https://fr.wikipedia.org/wiki/Ensemble_de_Julia

## Fonctionnalités
* Rendu dynamique de l'ensemble de Julia avec la bibliothèque minifb.
* Parallélisation des calculs sur tous les cœurs du processeur via la bibliothèque Rayon.
* Gestion adaptative de la résolution : la qualité augmente progressivement lorsque l'utilisateur s'arrête de naviguer.
* Commandes interactives pour le zoom, le déplacement et la modification du paramètre complexe c (parralele avec les conditions initiales).

---

## Installation

Pour compiler ce projet, vous devez disposer de l'environnement de développement Rust.

### 1. Installation de Rust (Cargo)
Rustup est l'outil standard pour installer Rust et son gestionnaire de paquets Cargo.
* **Site officiel :** [rust-lang.org](https://www.rust-lang.org/learn/get-started)
* **Linux / macOS :** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### 2. Dépendances système
Sur Linux, la bibliothèque **minifb** peut nécessiter l'installation des bibliothèques de développement X11 :
* **Ubuntu/Debian :** `sudo apt install libx11-dev`

---

## Exécution

Le projet utilise Cargo pour la gestion des dépendances et la compilation. Pour obtenir les meilleures performances de rendu, il est fortement recommandé d'utiliser le mode **release**.

```bash
cargo run --release
./target/release/julia
```

