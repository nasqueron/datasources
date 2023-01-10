//! Helper for items qualification.
//!
//! Wikidata uses the P31 "instance of" property to qualify items,
//! which is helpful to identify voies, especially the pseudo-voies
//! not furthermore described in FANTOIR.

use lazy_static::lazy_static;

lazy_static! {
    static ref P31_WINNERS: Vec<&'static str> = vec![
        // Important values

        "Q928830",     // metro station
        "Q18615527",   // tram bridge
        "Q1793804",    // station de RER
        "Q55488",      // gare ferroviaire
        "Q55485",      // gare ferroviaire en cul-de-sac

        "Q510662",     // ring road
        "Q2376564",    // échangeur autoroutier

        // Less important values, as probably already qualified by FANTOIR

        "Q3558430",    // villa, a name used for Paris private roads
        "Q15070223",   // cité, same thing

        "Q207934",     // allée
        "Q54114",      // boulevard
        "Q99228502",   // avenue (a road called avenue, not matching the avenue concept)
        "Q7543083",    // avenue (a true one)
        "Q283977",     // parvis
        "Q174782",     // place
        "Q164419",     // galerie

        "Q12731",      // impasse, shoud lose against avenue (some Paris avenues are so qualified)
        "Q13634881",   // passage
        "Q1251403",    // ruelle
        "Q3840711",    // quai
        "Q88372",      // esplanade, should win against jardin public
        "Q787113",     // promenade
        "Q17383262",   // cour
        "Q1068842",    // passerelle
        "Q641406",     // terrasse
        "Q16634966",   // escalier
        "Q628179",     // sentier
        "Q5004679",    // chemin
        "Q3352369",    // chemin piétonnier

        "Q1529",       // rond-point
        "Q1525",       // carrefour giratoire

        "Q4421",       // forêt, used for bois de Boulogne, bois de Vincennes
        "Q22698",      // parc
        "Q2026833",    // square, type jardin public
        "Q22746",      // jardin public
        "Q3215290",    // lac artificiel

        "Q12280",      // pont, should lost against place (large places at Paris are also bridges)
        "Q158438",     // pont en arc
        "Q537127",     // pont routier
        "Q1440300",    // tour d'observation

        "Q16560",      // palais
        "Q2080521",    // halle
        "Q16917",      // hôpital

        // Those values are probably too generic, so they're kept in last

        "Q1302778",    // voie rapide
        "Q79007",      // street, wins against road but loses against boulevard
        "Q83620",      // voie de communication
    ];
}

/// Determine amongst a sets of items which one is the more relevant
/// to describe a pseudo-voie.
///
/// This is useful when a Wikidata entity has several values for P31
/// to decide which one is the most interesting to keep in our context.
pub fn determine_p31_winner(candidates: &Vec<String>) -> String {
    if candidates.len() == 1 {
        // If there is only one, that's the one to use.
        return candidates[0].clone();
    }

    for winner_candidate in P31_WINNERS.iter() {
        for actual_candidate in candidates {
            if winner_candidate == actual_candidate {
                return actual_candidate.clone();
            }
        }
    }

    eprintln!("Can't determine P31 winner amongst {:?}, {} is picked.", candidates, candidates[0]);
    candidates[0].clone()
}
