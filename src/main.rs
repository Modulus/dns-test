use std::net::TcpStream;


use log::{info,error};
use env_logger::Env;

use openssl::ssl::{SslConnector, SslMethod};

fn main() {

    env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();


    info!("Verifying domains!");


    // let domains = [
    //     "admin.test.spasmodic.info",
    //     "api.test.spasmodic.info",
    //     "imove-no.test.spasmodic.info",
    //     "imove-se.test.spasmodic.info",
    //     "kinto-no.test.spasmodic.info",
    //     "schysst-se.test.spasmodic.info",
    //     "imove.no"
    // ];


    let domains = [
        "admin.dev.imove.no",
        "api.dev.imove.no",
        "imove-no.dev.imove.no",
        "imove-se.dev.imove.no",
        "kinto-no.dev.imove.no",
        "schysst-se.dev.imove.no"
    ];


    for domain in domains {
        info!("Verifying {}", domain);
        let connector = SslConnector::builder(SslMethod::tls()).expect("Failed to created ssl connector!").build();

        let stream_result = TcpStream::connect(format!("{}:443", String::from(domain)));

        match stream_result {
            Ok(stream) => {
                info!("{} resolved successfully!", domain);
                match connector.connect(domain, stream)  {
                    Ok(_) => {
                        info!("{} domain has valid ssl certificate!", domain)
                    },
                    Err(error) => {
                        error!("{} domain not valid ssl! Error: {}", domain, error);
                    }
                }
            },
            Err(error) => {
                error!("{} failed to resolve domain before ssl verification. Error: {}", domain, error);
            }
        }

    




    }

}
