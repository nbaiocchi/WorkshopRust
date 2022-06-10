# Workshop Rust

Petit projet rust fait pour appréhender les bases du language

Crée par `NoeBaiocchi` and `AmozPay`.

[![forthebadge](https://forthebadge.com/images/badges/built-with-love.svg)](https://forthebadge.com)

## Avant de commencer

L'objectif de ce workshop est d'aprendre les bases de Rust en suivant une certaine ligne directive pour ne pas etre perdu,
mais libre de modifié la thematique du projet de base pour faire autre chose qu'un programme de calcule mental :)

## Technologies
Le projet utilise:
* Rust
* Cargo
* Crates.io
	
## Setup

### Prerequis
* Install Rust 
	*  Run cet commande -> ```$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
	*  Site officiel -> `https://www.rust-lang.org/tools/install`

### Utils
* Documentation RustBook -> `https://doc.rust-lang.org/book/`
* Library/Package -> `https://crates.io/`


### Lancée le projet Exemple
```
$ open a terminal
$ clone the repository in your computer`
$ cd WORKSHOPRUST
$ cargo build
$ cargo run [CMD]
& cargo clippy
```
## Workshop I: Hello World

1) Utiliser Cargo pour initialiser votre projet
*	```$ cargo new [NAME] ```

2) Normalement vous aurez :
*	Un dossier src contenant un `main.rs`
*	Un dossier `target` utilisé pour generer et utiliser la documentation
*	Un fichier `Cargo.lock` pour utilisé pour les packages
*	Un fichier `Cargo.toml` que vous devez utiliser pour definir vos package/libs

3) Lancer le projet
* 	```$ cargo build``` -> Build votre projet pour voir si il compile
*	```$ cargo run [ARGS]``` -> Build et lance votre projet avec les possible arguments
*	```$ cargo clippy``` -> Verifier que votre programme est `Safe` celon les critéres de clippy


## Workshop II: Remplire

1) Recuperer le programme existant
*	git clone le projet

2) Completer
*	Maintenant vous devrais completer le programe donner afin de le rendre fonctionnel et optimisé
*	Toute les informations sont donné en dessous des `TODO :`
*	Veuillez completer par ce qui est demandé dans touts les emplacement ou la mention `/* A remplire */` est precisé

3) Créer votre propre commande
*	Maintenant que vous maitrisez un peu mieux rust vous devez créer votre propre commande en suivant les instruction donnée
	1. doit servir a resoudre des calcul simple (addition, multiplication, soustraction?)
	2. doit prendre 3 parametres non optionel 2 `int` (pour les valeurs) et un `char` (pour le symbole de calcul)
	3. ne doit pas etre hardcoder en `c` -> utiliser ce que rust met a disposition
	4. Doit etre `safe` et ne doit pas pouvoir crash (division par 0?) -> utilisez des tests pour montrer la sureté de votre code