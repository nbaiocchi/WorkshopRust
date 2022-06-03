use std::time::Duration;
use std::thread::sleep;

#[derive(PartialEq, Eq)]
pub enum TriState {
    Empty,
    Exit,
    Fine,
}

const EMPTY: TriState = TriState::Empty;
const EXIT: TriState = TriState::Exit;
const FINE: TriState = TriState::Fine;

// TODO :
//
// creer une fonction pour verifier la valeur entré par le joueur
//
// La fonction doit :
//  - check si `line` est vide ->                               return EMPTY
//  - check si le joueur veux quitter (si line = "exit") ->     return EXIT
//  - verifier si la reponse du joueur est corect ->            return FINE
//
// Pensait a bien verifier les cas d'erreur ;)
//
// DOC: https://doc.rust-lang.org/book/
//      https://www.google.com/
//
// prototype de la fonction : `pub fn check_res(res: i32, score: &mut i32, line: String) -> TriState`
//

pub fn check_res(res: i32, score: &mut i32, line: String) -> TriState {
    //* A remplire */,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exemple() {
        let res = 12;
        let line: String = "".to_string();

        match check_res(res, &mut 0, line) {
            EMPTY => (),
            _ => panic!("Should return a TriState::Empty"),
        }
    }

    // TODO :
    //
    // faire un test pour la return value EXIT de check_res comme l'exemple du dessus
    //
    // DOC: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
    //
    
    //* A remplire */,


    // TODO :
    //
    // faire un test pour la return value FINE de check_res en utilisant asser_eq
    //
    // DOC: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
    // 
    
    //* A remplire */,
}
