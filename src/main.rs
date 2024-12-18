use std::error::Error;
use std::fmt::{Debug, Formatter};
use tokio_tungstenite::{connect_async_tls_with_config};
use rustls::{ClientConfig, DigitallySignedStruct, SignatureScheme};
use tokio_tungstenite::Connector;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use futures_util::{SinkExt, StreamExt};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::frame::{Payload};
use rand::distributions::{Alphanumeric, DistString};

struct ServerCertVerifierImpl;

impl Debug for ServerCertVerifierImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ServerCertVerifier for ServerCertVerifierImpl{
    fn verify_server_cert(&self, _end_entity: &CertificateDer<'_>, _intermediates: &[CertificateDer<'_>], _server_name: &ServerName<'_>, _ocsp_response: &[u8], _now: UnixTime) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(&self, _message: &[u8], _cert: &CertificateDer<'_>, _dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(&self, _message: &[u8], _cert: &CertificateDer<'_>, _dss: &DigitallySignedStruct) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        Vec::from(
            [SignatureScheme::ED25519,
            SignatureScheme::ED448,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::RSA_PKCS1_SHA1,
            SignatureScheme::ECDSA_SHA1_Legacy])
    }
}

async fn connect_ws_with_tls(url: &str, count : Arc<AtomicUsize>) -> Result<(), Box<dyn Error>> {
    let config = ClientConfig::builder().dangerous().with_custom_certificate_verifier(Arc::new(ServerCertVerifierImpl)).with_no_client_auth();
    let connector = Connector::Rustls(Arc::new(config));

    match connect_async_tls_with_config(url,None, false, Some(connector)).await{
        Ok((mut ws_stream, _)) => {
            count.fetch_add(1, Ordering::SeqCst);
            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        println!("Received text: {}", text);
                    }
                    Ok(Message::Binary(data)) => {
                        if data.as_slice()[0] == 0x23 {
                            let _ = ws_stream.send(Message::Binary(Payload::Vec([0x1].to_vec()))).await;
                        }
                    }
                    Ok(Message::Ping(ping)) => {
                        println!("Received ping: {:?}", ping);
                        ws_stream.send(Message::Pong(ping)).await.unwrap();
                    }
                    Ok(Message::Pong(pong)) => {
                        println!("Received pong: {:?}", pong);
                    }
                    Ok(Message::Close(reason)) => {
                        println!("Connection closed: {:?}", reason);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Error while receiving message: {}", e);
                        break;
                    }
                    _ => {
                        println!("UNDEF ");
                    }
                }
            }
            count.fetch_sub(1, Ordering::SeqCst);
            println!("ws_disconnected {:?}", count);
        }
        Err(e) => {
            println!("ERROR {:?}", e.source())
        }
    };


    Ok(())
}

#[tokio::main]
async fn main() {
    let count = 60000;
    let conn_rate = 100;
    // 169.148.154.72:443
    let url : String = "wss://10.62.31.35:8201/ws/RT/1234/wt/<token>?user_id=<userid>_51&pub_channel=channel_1&sub_channels=channel_1&usc=channel_1&load_test=true".to_string();

    let mach_code = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);

    let mut success = Arc::new(AtomicUsize::new(0));
    let mut conn_interval = tokio::time::interval(Duration::from_secs(1));
    let mut cool_interval = tokio::time::interval(Duration::from_millis(12));
    conn_interval.tick().await;
    let mut connections = 0;
    cool_interval.tick().await;

    println!("{:?} | STARTING MGS LOAD TEST {mach_code} {count}", chrono::offset::Local::now());

    for i in 0..count {
        if connections >= conn_rate {
            println!("{:?} | LOAD STATE RAMP-UP : {:?}", chrono::offset::Local::now(), success);
            conn_interval.tick().await;
            connections = 0;
        }
        let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let mut temp_url = url.replace("<token>", token.as_str());
        temp_url = temp_url.replace("<userid>", format!("RT_x_{mach_code}_{i}").as_str());
        let c = success.clone();
        tokio::spawn(async move {
            let _ = connect_ws_with_tls(temp_url.as_str(), c).await;
        });
        cool_interval.tick().await;
        connections+=1;
    }

    conn_interval.tick().await;
    conn_interval.tick().await;
    conn_interval.tick().await;

    println!("{:?} | LOADING COMPLETED {count} {:?}", chrono::offset::Local::now(),  success);

    let mut monitor = tokio::time::interval(Duration::from_secs(30));
    loop {
        monitor.tick().await;
        println!("{:?} | CURRENT SESSIONS => {:?}", chrono::offset::Local::now(),  success);
    }
}
