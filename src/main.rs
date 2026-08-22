use log::debug;
use sanca_software::application::Application;

/* Ring used to be the default crypto provider in Rustls, but it is now
 * aws-lc. OpenBSD has strict rules about memory like W^X, and sometimes
 * Sanca receives a SIGSEGV signal. After checking the core dump, the issue
 * comes from aws-ls and doesn't happen with ring.
 * To avoid this issue in a clean way, by compiling with the feature
 * "ring-provider" it's possible to still use ring instead of aws-lc.
 */
#[cfg(not(feature = "ring-provider"))]
use rustls::crypto::aws_lc_rs as crypto_provider;

#[cfg(feature = "ring-provider")]
use rustls::crypto::ring as crypto_provider;

fn main() {
    //simple_logger::init_with_level(log::Level::Debug).unwrap();
	crypto_provider::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    debug!("Starting application");
    let mut application = Application::new();
    application.read_argv();
    application.run();
}
