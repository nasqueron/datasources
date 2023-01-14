use std::cmp::Ordering;
use std::path::Path;

use opendatasoft_explore_api::schema::Attachment;

use chrono::Datelike;
use chrono::Months;
use chrono::NaiveDate;

/*  -------------------------------------------------------------
    FANTOIR file metadata
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FantoirFile {
    pub url: String,

    /// The month of FANTOIR file production
    pub date: NaiveDate,
}

impl FantoirFile {
    pub fn from (attachment: &Attachment) -> Option<Self> {
        let id_date = attachment.metas.id
            .replace("fichier_national_fantoir_situation_", "")
            .replace("_zip", "");

        Some(Self {
            url: attachment.href.clone(),
            date: parse_fantoir_date(&id_date)?,
        })
    }

    pub fn get_file_candidates(&self) -> Vec<String> {
        let previous_month = self.date - Months::new(1);
        vec![
            format!("FANTOIR{}{}", previous_month.month(), previous_month.year() - 2000),
            format!("FANTOIR{}{}", self.date.month(), self.date.year() - 2000),
        ]
    }

    pub fn exists_locally(&self) -> bool {
        self.get_file_candidates()
            .iter()
            .any(|candidate| Path::new(candidate).is_file())
    }
}

impl PartialOrd for FantoirFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FantoirFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

fn parse_fantoir_date(id_date: &str) -> Option<NaiveDate> {
    let parts: Vec<_> = id_date.split("_").collect(); // [ month in French, year ]

    if parts.len() != 2 {
        return None;
    }

    NaiveDate::from_ymd_opt(
        parts[1].parse().ok()?,
        parse_french_month_long_name(parts[0])?,
        1
    )
}

fn parse_french_month_long_name(month: &str) -> Option<u32> {
    match month {
        "janvier" => Some(1),
        "fevrier" => Some(2),
        "mars" => Some(3),
        "avril" => Some(4),
        "mai" => Some(5),
        "juin" => Some(6),
        "juillet" => Some(7),
        "aout" => Some(8),
        "septembre" => Some(9),
        "octobre" => Some(10),
        "novembre" => Some(11),
        "decembre" => Some(12),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use super::*;

    #[test]
    fn test_parse_fantoir_date() {
        let expected = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        assert_eq!(Some(expected), parse_fantoir_date("novembre_2022"));
    }

    #[test]
    fn test_get_file_candidates() {
        let file = FantoirFile {
            url: "foo/fichier_national_fantoir_situation_novembre_2022_zip".to_string(),
            date: NaiveDate::from_ymd_opt(2022, 11, 1).unwrap(),
        };

        let expected = vec!["FANTOIR1022".to_string(), "FANTOIR1122".to_string()];
        assert_eq!(expected, file.get_file_candidates());
    }
}
