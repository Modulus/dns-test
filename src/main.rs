
use std::net::TcpStream;
use std::env;
use dns_test::types::CertificateMetadata;
use log::{info,error};
use env_logger::Env;

use openssl::ssl::{SslConnector, SslMethod};

fn main() {

    env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();


    info!("Verifying domains!");


    let domains = extract_domains();

    let mut metadata : Vec<CertificateMetadata> = Vec::new();
    for domain in domains {
        let domain_str = domain.as_str();
        info!("Verifying {}", domain);
        let connector = SslConnector::builder(SslMethod::tls()).expect("Failed to created ssl connector!").build();

        let stream_result = TcpStream::connect(format!("{}:443", domain));

        match stream_result {
            Ok(stream) => {
                info!("{} resolved successfully!", domain_str);
                match connector.connect(domain_str, stream)  {
                    Ok(ssl_stream) => {
                        info!("{} domain has valid ssl certificate!", domain_str);
                        let certificate = ssl_stream.ssl().peer_certificate().ok_or("Certificate not found").expect("Failed to extract certificated data!");
                        info!("{:?}", certificate.not_after());
                        info!("{:?}", certificate.not_before());

                        let meta = CertificateMetadata{domain: domain_str.to_string(), not_after: certificate.not_after().to_string(), not_before: certificate.not_before().to_string()};
                        metadata.push(meta);

                    },
                    Err(error) => {
                        error!("{} domain not valid ssl! Error: {}", domain_str, error);
                    }
                }
            },
            Err(error) => {
                error!("{} failed to resolve domain before ssl verification. Error: {}", domain_str, error);
            }
        }


    }

}

pub fn extract_domains() -> Vec<String>{
    let mut domains : Vec<String> = Vec::new();

    let domain_var = env::var("DOMAINS").expect("You must set the DOMAIN env var with comma separated values. Ie 'test.net,test2.com'");

    let split_list = domain_var.split(",");

    for str in split_list {
        if !str.is_empty(){
            domains.push(String::from(str));
        }
    }



    return domains;

}


#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    #[test]
    fn test_domain_env_var(){
        let domains = "vg.no,itavisen.no,dagbladet.no,".to_string();

        env::set_var("DOMAINS", domains);

        let result = extract_domains();


        assert!(result.len() == 3);
        assert!(result.contains(&String::from("vg.no")));
        assert!(result.contains(&String::from("itavisen.no")));
        assert!(result.contains(&String::from("dagbladet.no")));
        assert!(result.contains(&String::from("")) == false);



    }
}