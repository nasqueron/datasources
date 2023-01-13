use std::cmp::Ordering;
use std::collections::HashMap;

use sqlx::Error;

use crate::commands::wikidata::WikidataEntryKey;

type MaintenanceReport = HashMap<&'static str, Vec<WikidataEntryKey>>;

/*   -------------------------------------------------------------
     Report update and wiki code
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub fn update_report (maintenance_report: &mut MaintenanceReport, key: WikidataEntryKey, error: Error) {
    let error_category = match error {
        Error::Database(error) => {
            if let Some(index) = error.constraint() {
                match index {
                    "index_fantoir_wikidata_pk" => "Duplicate FANTOIR code",
                    "fantoir_wikidata_code_fantoir_fk" => "Not in FANTOIR national file",
                    _ => {
                        eprintln!("Unknown constraint index: {}", index);

                        unreachable!()
                    },
                }
            } else if let Some(code) = error.code() {
                let code = code.to_string();
                match code.as_str() {
                    "22001" => "FANTOIR code too long",
                    _ => unimplemented!(),
                }
            } else {
                unimplemented!()
            }
        },
        _ => unimplemented!(),
    };

    let entry = maintenance_report
        .entry(error_category)
        .or_insert(Vec::new());
    entry.push(key);
}

pub fn print_maintenance_report (maintenance_report: MaintenanceReport) {
    for (section_title, mut entries) in maintenance_report {
        println!("== {} ==", section_title);
        println!(r#"
{{| class="wikitable sortable"
|+ Items with issue
|-
! Item !! Item label in French !! FANTOIR code"#);

        entries.sort();
        for entry in entries {
            println!(r#"|-
| [[{}]] || {} || {}"#, &entry.item, &entry.item_label, &entry.code_fantoir_wikidata);
        }

        println!(r#"|}}"#);
        println!();
    }

    println!("== Notes ==");
    println!("This maintenance report has been generated automatically by fantoir-datasource tool, based on the issues encountered to cross-validate Wikidata entries and FANTOIR national file.");
}

/*   -------------------------------------------------------------
     Sort for report entries
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

impl PartialOrd for WikidataEntryKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WikidataEntryKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.code_fantoir_wikidata.cmp(&other.code_fantoir_wikidata)
    }
}
