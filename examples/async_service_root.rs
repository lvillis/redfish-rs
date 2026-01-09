use redfish::{Auth, Client};

#[tokio::main]
async fn main() -> Result<(), redfish::Error> {
    let client = Client::builder("https://bmc.example.com")?
        .auth(Auth::basic("admin", "password"))
        .build()?;

    let root = client.service_root().get().await?;
    println!("RedfishVersion: {}", root.redfish_version);

    Ok(())
}
