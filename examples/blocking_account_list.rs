//! List Redfish accounts (blocking client).

use redfish::{Auth, BlockingClient};

fn main() -> Result<(), redfish::Error> {
    let client = BlockingClient::builder("https://bmc.example.com")?
        .auth(Auth::basic("admin", "password"))
        .build()?;

    let accounts = client.account_service().list_accounts()?;
    for m in accounts.members {
        println!("Account: {}", m.odata_id);
    }

    Ok(())
}
