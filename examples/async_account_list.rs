//! List Redfish accounts.

use redfish::{Auth, Client};

#[tokio::main]
async fn main() -> Result<(), redfish::Error> {
    let client = Client::builder("https://bmc.example.com")?
        .auth(Auth::basic("admin", "password"))
        .build()?;

    let accounts = client.account_service().list_accounts().await?;
    for m in accounts.members {
        println!("Account: {}", m.odata_id);
    }

    Ok(())
}
