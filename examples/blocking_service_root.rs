use redfish::{Auth, BlockingClient};

fn main() -> Result<(), redfish::Error> {
    let client = BlockingClient::builder("https://bmc.example.com")?
        .auth(Auth::basic("admin", "password"))
        .build()?;

    let root = client.service_root().get()?;
    println!("RedfishVersion: {}", root.redfish_version);

    Ok(())
}
