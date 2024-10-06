use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = "3762a9c0c658245e7c5e8611ed553a4b";
    let base_url = "http://api.marketstack.com/v1/eod";

    // Parámetros de la consulta
    let query_params = [
        ("access_key", api_key),
        ("symbols", "AAPL"),
        ("date_from", "2024-09-26"),
        ("date_to", "2024-09-27"),
    ];

    // Crea el cliente HTTP
    let client = reqwest::Client::new();

    // Realiza la solicitud GET con los parámetros de consulta
    let res = client
        .get(base_url)
        .query(&query_params)
        .send()
        .await?
        .text()
        .await?;

    // Imprime la respuesta como texto
    println!("{}", res);

    Ok(())
}
