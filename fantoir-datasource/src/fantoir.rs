//! # Helper methods for FANTOIR database.
//!
//! This module offers a structure for a FANTOIR record, methods to parse the file and export it.
//! Database functions expect to work with an executor from sqlx crate.

use lazy_static::lazy_static;
use sqlx::PgPool;
use sqlx::types::chrono::NaiveDate;

lazy_static! {
    static ref DEPARTMENTS_WITH_CODE_DIRECTION: Vec<&'static str> = vec!["13", "59", "75", "92", "97"];

    /// The alphabet without I O and Q.
    static ref RIVOLI_STRING: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M',
        'N', 'P', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];
}

/// A voie in the FANTOIR database
#[derive(Debug)]
pub struct FantoirEntry {
    /* Identifiers */
    code_fantoir: String,

    /* Part 1 - commune */
    departement: String, // Generally an integer, but INSEE uses 2A and 2B for Corse
    code_commune: i32,
    code_insee: String,  // Afa in Corse has 2A001
    type_commune: Option<String>,
    is_pseudo_recensee: bool,

    /* Part 2 - voie */
    identifiant_communal_voie: String,
    cle_rivoli: String,
    code_nature_voie: Option<String>,
    libelle_voie: String,
    type_voie: i32, // 1: voie, 2: ens. immo, 3: lieu-dit, 4: pseudo-voie, 5: provisoire
    is_public: bool,

    /* Part 3 - population */
    is_large: bool,
    population_a_part: i32,
    population_fictive: i32,

    /* Part 4 - metadata */
    is_cancelled: bool,
    cancel_date: Option<NaiveDate>,
    creation_date: Option<NaiveDate>,
    code_majic: i32,
    last_alpha_word: String,
}

impl FantoirEntry {
    pub fn parse_line(line: &str) -> Self {
        let departement = match &line[0..2] {
            "97" => String::from(&line[0..3]), // include for DOM/TOM the next digit
            department => String::from(department),
        };
        let len = line.len();

        Self {
            /* Identifier */
            code_fantoir: String::from(&line[0..11]),

            /* Part 1 - commune */
            departement,
            code_commune: line[3..6].parse().expect("Can't parse code commune"),
            code_insee: format!("{:02}{:03}", &line[0..2], &line[3..6]),
            type_commune: parse_optional_string(&line[43..44]),
            is_pseudo_recensee: &line[45..46] == "3",

            /* Part 2 - voie */
            identifiant_communal_voie: String::from(&line[6..10]),
            cle_rivoli: String::from(&line[10..11]),
            code_nature_voie: parse_optional_string(&line[11..15]),
            libelle_voie: String::from(line[15..41].trim()),
            type_voie: line[108..109].parse().expect("Can't parse type de voie."),
            is_public: &line[48..49] == "0",

            /* Part 3 - population */
            is_large: &line[49..50] == "*",
            population_a_part: line[59..66].parse().expect("Can't parse population à part"),
            population_fictive: line[66..73].parse().expect("Can't parse population fictive"),

            /* Part 4 - metadata */
            is_cancelled: &line[73..74] != " ",
            cancel_date: parse_fantoir_date(&line[74..81]),
            creation_date: parse_fantoir_date(&line[81..88]),
            code_majic: line[103..108].parse().expect("Can't parse MAJIC"),
            last_alpha_word: String::from(&line[112..len]),
        }
    }

    pub async fn insert_to_db(&self, pool: &PgPool, table: &str) {
        let mut query = format!("INSERT INTO {}", table);
        query.push_str(
            r#"
            (code_fantoir,
             departement, code_commune, code_insee, type_commune, is_pseudo_recensee,
             identifiant_communal_voie, cle_rivoli, code_nature_voie, libelle_voie, type_voie, is_public,
             is_large, population_a_part, population_fictive,
             is_cancelled, cancel_date, creation_date, code_majic, last_alpha_word
            )
        VALUES
            ($1,
             $2, $3, $4, $5, $6,
             $7, $8, $9, $10, $11, $12,
             $13, $14, $15,
             $16, $17, $18, $19, $20
            )"#
        );

        sqlx::query(&query)
            /* Identifiers */
            .bind(&self.code_fantoir)

            /* Part 1 - commune */
            .bind(&self.departement)
            .bind(&self.code_commune)
            .bind(&self.code_insee)
            .bind(&self.type_commune)
            .bind(&self.is_pseudo_recensee)

            /* Part 2 - Voie */
            .bind(&self.identifiant_communal_voie)
            .bind(&self.cle_rivoli)
            .bind(&self.code_nature_voie)
            .bind(&self.libelle_voie)
            .bind(&self.type_voie)
            .bind(&self.is_public)

            /* Part 3 - Population */
            .bind(&self.is_large)
            .bind(&self.population_a_part)
            .bind(&self.population_fictive)

            /* Part 4 - Metadata */
            .bind(&self.is_cancelled)
            .bind(&self.cancel_date)
            .bind(&self.creation_date)
            .bind(&self.code_majic)
            .bind(&self.last_alpha_word)

            .execute(pool)
            .await
            .expect("Can't insert entry to database");
    }
}

pub fn parse_fantoir_date (date: &str) -> Option<NaiveDate> {
    if date == "0000000" {
        return None;
    }

    let year = date[0..4].parse().expect("Can't parse date: year part");
    let ord = date[4..7].parse().expect("Can't parse date: ordinal part");

    NaiveDate::from_yo_opt(year, ord)
}

fn parse_optional_string (expression: &str) -> Option<String> {
    let expression = expression.trim();

    if expression.len() > 0 {
        Some(String::from(expression))
    } else {
        None
    }
}

/// A fixed FANTOIR code result
#[derive(Debug, Eq, PartialEq)]
pub enum FixedFantoirCode {
    /// The code has been fully computed
    Computed(String),

    /// Information needed to query the code has been extracted, but code direction is unknown
    /// Such result can be queried through search_code_fantoir()
    ToSearch { code_insee: String, identifiant_communal_voie: String },
}

/// Transforms FANTOIR code from BAN into regular FANTOIR codes.
/// BAN sometimes uses <insee code>_<identifiant voie commune> without Rivoli key.
pub fn fix_fantoir_code(code: &str) -> FixedFantoirCode {
    let mut code = code.to_string();

    if code.contains("_") {
        // 97231_B026 -> 972231B026
        code = if code.starts_with("97") {
            // Code direction = department last digit
            format!("{}{}{}", &code[0..=2], &code[2..5], &code[6..])
        } else if uses_specific_code_direction(&code) {
            // We can't fix it by computation, we need to search it in the database
            return FixedFantoirCode::ToSearch {
                code_insee: code[0..5].to_string(),
                identifiant_communal_voie: code[6..10].to_string(),
            }
        } else {
            // Code direction = 0
            format!("{}0{}{}", &code[0..=2], &code[3..5], &code[6..])
        };
    }

    if code.len() == 10 {
        let last_char = code.chars().last().unwrap();

        match last_char {
            '0'..='9' => {
                code.push(compute_rivoli_key(&code));
            }

            'A'..='Z' => {
                // 441090516U -> 4401090516U
                code = if uses_specific_code_direction(&code) {
                    // We can't fix it by computation, we need to search it in the database
                    // 920514135A -> 92051 4135
                    return FixedFantoirCode::ToSearch {
                        code_insee: code[0..5].to_string(),
                        identifiant_communal_voie: code[5..9].to_string(),
                    }
                } else {
                    format!("{}0{}", &code[0..2], &code[2..])
                };
            }

            _ => unreachable!(),
        }
    }

   FixedFantoirCode::Computed(code)
}

pub fn uses_specific_code_direction (code: &str) -> bool {
    DEPARTMENTS_WITH_CODE_DIRECTION
        .iter()
        .any(|&dpt| code.starts_with(dpt))
}

pub fn compute_rivoli_key (code: &str) -> char {
    // See https://georezo.net/forum/viewtopic.php?id=102292

    if code.starts_with("2A") || code.starts_with("2B") {
        // 2A would be 2 10 and 2B would be 2 11, but how to build a number to multiply by 19?
        unimplemented!()
    }

    let part_commune: i32 = code[0..6].parse().unwrap();
    let type_voie = code.chars().nth(6).unwrap();
    let type_voie = if type_voie.is_alphabetic() {
        type_voie as u32 - 55
    } else {
        type_voie.to_digit(10).unwrap()
    };
    let numero_identifiant_communal_voie: i32 = code[7..].parse().unwrap();

    let index = (part_commune * 19 + type_voie as i32 * 11 + numero_identifiant_communal_voie) % 23;
    return RIVOLI_STRING[index as usize];
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_fantoir_date() {
        let expected = NaiveDate::from_ymd_opt(1987, 1, 1).unwrap();
        let actual = parse_fantoir_date("1987001").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_optional_string() {
        assert_eq!(Some(String::from("quux")), parse_optional_string("quux"));
    }

    #[test]
    fn test_parse_optional_string_with_trailing_spaces() {
        assert_eq!(Some(String::from("quux")), parse_optional_string("quux    "));
    }

    #[test]
    fn test_parse_optional_string_when_empty() {
        assert_eq!(true, parse_optional_string("").is_none());
    }

    #[test]
    fn test_parse_optional_string_when_only_spaces() {
        assert_eq!(true, parse_optional_string("    ").is_none());
    }

    #[test]
    pub fn test_fix_fantoir_code () {
        assert_fixed_fantoir_code("755112P144L", fix_fantoir_code("755112P144L"));
        assert_fixed_fantoir_code("972231B026U", fix_fantoir_code("97231_B026"));
        assert_fixed_fantoir_code("4401090516U", fix_fantoir_code("441090516U"));
        assert_fixed_fantoir_code("972222B305L", fix_fantoir_code("972222B305"));
    }

    fn assert_fixed_fantoir_code (expected: &str, actual: FixedFantoirCode) {
        match actual {
            FixedFantoirCode::Computed(code) => {
                assert_eq!(expected, &code);
            },
            _ => assert!(false, "Expected a computed FANTOIR code")
        }
    }

    #[test]
    pub fn test_fix_fantoir_code_when_it_cannot_be_computed () {
        let expected = FixedFantoirCode::ToSearch {
            code_insee: "92002".to_string(),
            identifiant_communal_voie: "5130".to_string()
        };

        assert_eq!(expected, fix_fantoir_code("920025130X"), "As code direction can't be computed, this code should be to search");
        assert_eq!(expected, fix_fantoir_code("92002_5130"), "As code direction can't be computed, this code should be to search");
    }


    #[test]
    pub fn test_compute_rivoli_key() {
        assert_eq!('W', compute_rivoli_key("380003B001"));
        assert_eq!('U', compute_rivoli_key("972231B026"));
    }

    #[test]
    pub fn test_compute_rivoli_key_with_type_voie_zero() {
        assert_eq!('C', compute_rivoli_key("9722230261"));
    }
}
