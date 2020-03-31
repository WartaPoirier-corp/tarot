#[derive(Clone, PartialEq, Eq)]
enum Couleur {
    Carreau,
    Pique,
    Trefle,
    Coeur,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Valeur {
    As,
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
    Sept,
    Huit,
    Neuf,
    Dix,
    Valet,
    Cavalier,
    Dame,
    Roi,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Atout {
    Excuse,
    Un,
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
    Sept,
    Huit,
    Neuf,
    Dix,
    Onze,
    Douze,
    Treize,
    Quatorze,
    Quinze,
    Seize,
    DixSept,
    Dixhuit,
    Dixneuf,
    Vingt,
    VingEtUn,
}

#[derive(Clone, Eq, PartialEq)]
enum Carte {
    CarteNorm(Couleur, Valeur),
    Atout(Atout),
}

impl Carte {
    fn plus_forte_que(&self, autre: &Carte, appelee: Couleur) -> bool {
        match self {
            Carte::CarteNorm(coul, val) => match autre {
                Carte::Atout(_) => false,
                Carte::CarteNorm(coul_autre, val_autre) => {
                    if *coul == *coul_autre {
                        assert!(*val != *val_autre);
                        *val > *val_autre
                    } else {
                        appelee == *coul
                    }
                }
            },
            Carte::Atout(at) => match autre {
                Carte::CarteNorm(_, _) => true,
                Carte::Atout(autre_at) => *at > *autre_at,
            },
        }
    }
}

type Joueur = String;

fn couleur_demandee(cartes: &[Carte]) -> Option<Couleur> {
    if let Carte::CarteNorm(coul, _) = cartes[0].clone() {
        Some(coul)
    } else {
        None
    }
}

fn a_couleur(cartes: &[Carte], couleur: Option<Couleur>) -> bool {
    for c in cartes {
        match c {
            Carte::CarteNorm(c, _) => {
                if let Some(coul_demandee) = couleur.clone() {
                    if *c == coul_demandee {
                        return true;
                    }
                }
            }
            Carte::Atout(_) => {
                if couleur.is_none() {
                    return true;
                }
            }
        }
    }
    return false;
}

/// Donne le plus grand atout d'une liste de carte si il existe
fn atout_max(cartes: &[Carte]) -> Option<Atout> {
    cartes.iter().fold(None, |max, carte| match carte {
        Carte::CarteNorm(_, _) => max,
        Carte::Atout(at) => {
            if let Some(at_max) = max.clone() {
                if *at > at_max {
                    Some(at.clone())
                } else {
                    max.clone()
                }
            } else {
                max.clone()
            }
        }
    })
}

fn gagnant_de_tour(cartes: &[(Joueur, Carte)]) -> Joueur {
    let demandee = couleur_demandee(&cartes.iter().map(|x| x.1.clone()).collect::<Vec<_>>()[..]);
    cartes
        .iter()
        .fold(cartes[0].clone(), |gagnant, joueur| {
            if gagnant
                .1
                .plus_forte_que(&joueur.1, demandee.clone().unwrap_or(Couleur::Carreau))
            {
                gagnant
            } else {
                joueur.clone()
            }
        })
        .0
}

#[test]
fn test_gagant() {}

fn cartes_jouables(
    cartes_jouees: &[Carte],
    cartes_joueur: &[Carte],
    premier_tour: bool,
    roi_appele: Couleur,
) -> Vec<Carte> {
    cartes_joueur.iter().filter(|carte| {
    if let Carte::Atout(Atout::Excuse) = carte {
      return true;
    }

    if premier_tour && cartes_jouees.len() == 0 {
      match carte {
        Carte::CarteNorm(coul, val) => (*coul != roi_appele) || (*val == Valeur::Roi),
        Carte::Atout(_) => true,
      }
    } else {
      let couleur_dem = couleur_demandee(cartes_jouees);
      if let Some(coul) = couleur_dem {
        if a_couleur(cartes_joueur, Some(coul.clone())) {
          match carte {
            Carte::CarteNorm(c,_) => *c == coul,
            _ => false,
          }
        } else {
          if a_couleur(cartes_joueur, None) {
            match carte {
              Carte::Atout(val) => if let Some(max) = atout_max(cartes_jouees) {
                if let Some(atout_max_joueur) = atout_max(cartes_joueur) {
                  if atout_max_joueur > max {
                    *val > max
                  } else {
                    true
                  }
                } else {
                  unreachable!("Pas d'atout max mais un atout quand même. Ça pulse pas.")
                }
              } else {
                true
              },
              _ => false,
            }
          } else {
            true
          }
        }
      } else {
        if a_couleur(cartes_joueur, None) {
          match carte {
            Carte::Atout(val) => if let Some(max) = atout_max(cartes_jouees) {
              if let Some(atout_max_joueur) = atout_max(cartes_joueur) {
                if atout_max_joueur > max {
                  *val > max
                } else {
                  true
                }
              } else {
                unreachable!("Pas d'atout max mais un atout quand même. Ça pulse pas.")
              }
            } else {
              true
            },
            _ => false,
          }
        } else {
          true
        }
      }
    }
  }).map(|x| x.clone()).collect()
}

/// Crée un jeu de carte, avec les cartes classées.
fn creer_jeu() -> Vec<Carte> {
    let mut jeu = Vec::with_capacity(78);
    for coul in [
        Couleur::Carreau,
        Couleur::Coeur,
        Couleur::Trefle,
        Couleur::Pique,
    ]
    .iter()
    {
        for val in [
            Valeur::As,
            Valeur::Deux,
            Valeur::Trois,
            Valeur::Quatre,
            Valeur::Cinq,
            Valeur::Six,
            Valeur::Sept,
            Valeur::Huit,
            Valeur::Neuf,
            Valeur::Dix,
            Valeur::Valet,
            Valeur::Cavalier,
            Valeur::Dame,
            Valeur::Roi,
        ]
        .iter()
        {
            jeu.push(Carte::CarteNorm(coul.clone(), val.clone()));
        }
    }

    for atout in [
        Atout::Un,
        Atout::Deux,
        Atout::Trois,
        Atout::Quatre,
        Atout::Cinq,
        Atout::Six,
        Atout::Sept,
        Atout::Huit,
        Atout::Neuf,
        Atout::Dix,
        Atout::Onze,
        Atout::Douze,
        Atout::Treize,
        Atout::Quatorze,
        Atout::Quinze,
        Atout::Seize,
        Atout::DixSept,
        Atout::Dixhuit,
        Atout::Dixneuf,
        Atout::Vingt,
        Atout::VingEtUn,
        Atout::Excuse,
    ]
    .iter()
    {
        jeu.push(Carte::Atout(atout.clone()));
    }
    jeu
}

fn input(msg: &str) -> String {
    println!("{}", msg);
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).unwrap();
    res.trim().to_string()
}

fn main() {
    println!("On va jouer au TAROT !!!!");
    let _nb_joueur: i32 = 5; //input("Combien de joueurs ?").parse().unwrap();
    let jeu = creer_jeu();
    if jeu.len() == 78 && jeu[10] == Carte::CarteNorm(Couleur::Carreau, Valeur::Valet) {
        println!("Wow ça pulse");
    }
    let atout_vingt = Carte::Atout(Atout::Vingt);
    let atout_dix = Carte::Atout(Atout::Dix);
    let roi_de_coeur = Carte::CarteNorm(Couleur::Coeur, Valeur::Roi);
    if atout_vingt.plus_forte_que(&atout_dix, Couleur::Carreau) {
        println!("Wow ça pulse");
    }

    if couleur_demandee(&[roi_de_coeur, atout_dix]) == Some(Couleur::Coeur) {
        println!("Wow ça pulse");
    }

    let jouables = cartes_jouables(
        &[jeu[2].clone(), jeu[5].clone(), jeu[6].clone()],
        &[
            jeu[1].clone(),
            jeu[20].clone(),
            jeu[50].clone(),
            jeu[60].clone(),
        ],
        false,
        Couleur::Carreau,
    );
    if jouables == vec![jeu[1].clone()] {
        println!("Wow ça pulse (si on peut jouer la couleur demandée)");
    }

    let jouables = cartes_jouables(
        &[jeu[2].clone(), jeu[5].clone(), jeu[6].clone()],
        &[
            jeu[1].clone(),
            jeu[20].clone(),
            jeu[50].clone(),
            jeu[60].clone(),
            jeu[77].clone(),
        ],
        false,
        Couleur::Carreau,
    );
    if jouables == vec![jeu[1].clone(), jeu[77].clone()] {
        println!("Wow ça pulse (si on peut jouer l'excuse)");
    }

    if gagnant_de_tour(&[
        ("Johan".into(), jeu[60].clone()),
        ("Mathis".into(), jeu[20].clone()),
        ("Clara".into(), jeu[10].clone()),
        ("Pénélope".into(), jeu[30].clone()),
        ("Théo".into(), jeu[1].clone()),
    ]) == "Johan".to_owned()
    {
        println!("Wow ça pulse");
    }
}
