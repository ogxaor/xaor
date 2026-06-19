use xaor::engine::xaor::XaorEngine;
use xaor::{config::XaorMode, XaorConfig};

fn main() {
    let mut config = XaorConfig::default();
    config.mode = XaorMode::Encrypt;
    let engine = XaorEngine::new(config).unwrap();

    let result = engine.process(b"xaor-test", None).unwrap();

    println!("Final length: {}", result.len());
    println!("{:?}", result);
}
