extern crate football_data;

use football_data::base_response::BaseResponse;
use football_data::client::FootballDataClient;
use football_data::competition::Competitions;
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_client() -> Result<(), reqwest::Error> {
    let mock_server = MockServer::start().await;
    let client = FootballDataClient::new(Option::from(mock_server.uri()), None);

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"Count": 0, "Competitions": []}"#.as_bytes().to_owned(),
            "application/json",
        ))
        .expect(1)
        .mount(&mock_server)
        .await;

    let expected: Competitions = Competitions {
        base_response: BaseResponse { count: 0 },
        competitions: vec![],
    };

    let competitions = client.get_competitions().await?;
    assert_eq!(expected, competitions);
    Ok(())
}
