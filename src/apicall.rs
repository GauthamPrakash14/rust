use reqwest::Error;

#[tokio::main]
pub async fn api() -> Result<(), Error> {
    // Specify the URL of the API endpoint
    let url = "https://api.spacexdata.com/v5/launches/latest";

    // Send a GET request to the API endpoint
    let response = reqwest::get(url).await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Read the response body as a string
        let body = response.text().await?;
        println!("Response body: {}", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
