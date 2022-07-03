use crate::*;

pub(crate) fn expect_lightweight<T>(
    option: Option<T>,
    message: &str
) -> T {
    option.unwrap_or_else(|| env::panic_str(message))
}


// #[derive(Deserialize, Debug)]
// pub(crate) struct API {

// }


// #[derive(Deserialize, Debug)]
// pub(crate) struct Response {

// }

// #[tokio::main]
// pub(crate) async fn check_validity_api(url: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let resp: API = reqwest::get(url)
//         .await?
//         .json()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

// pub(crate) fn check_validity_api(url: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let resp: API = reqwest::get(url)?.json()?;
//     println!("{:#?}", resp);
//     Ok(())
// }



// pub(crate) fn test_code(url: &str) {
    // let request = ehttp::Request::get(url);
    // ehttp::fetch(request, |result: ehttp::Result<ehttp::Response>| {
    //     env::log_str(format!(
    //         "{:#?}",
    //         result.unwrap()
    //     ).as_str());
    // });
// }