use xaor::VaultEngine;

fn main() {
    let vault = VaultEngine::new("target/xaor-example.vault");
    let key = b"0123456789abcdef0123456789abcdef";

    vault.store("api_key", b"secret-token", key).unwrap();
    let retrieved = vault.retrieve("api_key", key).unwrap().unwrap();

    println!("Retrieved: {}", String::from_utf8_lossy(&retrieved));
    println!("All entries: {:?}", vault.list().unwrap());

    vault.delete("api_key").unwrap();
}
