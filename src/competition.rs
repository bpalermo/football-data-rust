use crate::base_response::BaseResponse;

pub(crate) const COMPETITIONS_ENDPOINT: &str = "/v4/competitions";

#[derive(Eq, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CompetitionType {
    LEAGUE,
}

#[derive(Eq, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Competitions {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    #[serde(rename(deserialize = "Competitions"))]
    pub competitions: Vec<Competition>,
}

#[derive(Eq, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Competition {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub r#type: CompetitionType,
    pub emblem: String,
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::competition::{Competition, CompetitionType};

    #[test]
    fn test_deserialize() {
        let payload = r#"
            {
                "area": {
                    "id": 2072,
                    "name": "England",
                    "code": "ENG",
                    "flag": "https://crests.football-data.org/770.svg"
                },
                "id": 2021,
                "name": "Premier League",
                "code": "PL",
                "type": "LEAGUE",
                "emblem": "https://crests.football-data.org/PL.png",
                "currentSeason": {
                    "id": 733,
                    "startDate": "2021-08-13",
                    "endDate": "2022-05-22",
                    "currentMatchday": 37,
                    "winner": null,
                    "stages": [
                        "REGULAR_SEASON"
                    ]
                },
                "seasons": [
                    {
                        "id": 733,
                        "startDate": "2021-08-13",
                        "endDate": "2022-05-22",
                        "currentMatchday": 37,
                        "winner": null,
                        "stages": [
                            "REGULAR_SEASON"
                        ]
                    },
                    {
                        "id": 619,
                        "startDate": "2020-09-12",
                        "endDate": "2021-05-23",
                        "currentMatchday": 38,
                        "winner": {
                            "id": 65,
                            "name": "Manchester City FC",
                            "shortName": "Man City",
                            "tla": "MCI",
                            "crest": "https://crests.football-data.org/65.png",
                            "address": "SportCity Manchester M11 3FF",
                            "website": "https://www.mancity.com",
                            "founded": 1880,
                            "clubColors": "Sky Blue / White",
                            "venue": "Etihad Stadium",
                            "lastUpdated": "2022-02-10T19:48:37Z"
                        },
                        "stages": [
                            "REGULAR_SEASON"
                        ]
                    }
                ],
                "lastUpdated": "2022-03-20T08:58:54Z"
            }
        "#;

        let expected = Competition {
            id: 2021,
            name: String::from("Premier League"),
            code: String::from("PL"),
            r#type: CompetitionType::LEAGUE,
            emblem: String::from("https://crests.football-data.org/PL.png"),
        };
        let c: Competition = from_str(payload).unwrap();
        assert_eq!(expected, c)
    }
}
