use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use bollard::{Docker, container::{Config, CreateContainerOptions, StartContainerOptions}, models::{HostConfig, PortBinding, ContainerCreateResponse}};
use rand::{distributions::Alphanumeric, Rng};
use rsa::{pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding}, RsaPrivateKey};
use std::{collections::HashMap, time::Duration};

#[derive(Serialize)]
struct SshLaunch {
    host: String,
    port: u16,
    user: String,
    private_key_pem: String,
}

#[derive(Deserialize)]
struct LaunchReq {
    learner: String,
}

async fn launch_handler(q: web::Query<LaunchReq>) -> actix_web::Result<impl Responder> {
    let info = launch_ssh_pod(&q.learner).await.map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(info))
}

async fn launch_ssh_pod(learner_id: &str) -> Result<SshLaunch> {
    let key = RsaPrivateKey::new(&mut rand::thread_rng(), 2048)?;
    let priv_pem = key.to_pkcs8_pem(LineEnding::LF)?.to_string();
    let pub_ssh = key
        .to_public_key()
        .to_public_key_pem(LineEnding::LF)?
        .replace("-----BEGIN PUBLIC KEY-----\n", "")
        .replace("-----END PUBLIC KEY-----", "");

    let cname: String = format!(
        "sim-{}-{}",
        learner_id,
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect::<String>()
    );

    let docker = Docker::connect_with_socket_defaults()?;
    let create_cfg = Config {
        image: Some("robot-sim:latest".into()),
        env: Some(vec![format!("SSH_PUBKEY={}", pub_ssh)]),
        exposed_ports: Some(
            [("2222/tcp".into(), HashMap::<(), ()>::new())]
                .iter()
                .cloned()
                .collect(),
        ),
        host_config: Some(HostConfig {
            port_bindings: Some(
                [(
                    "2222/tcp".into(),
                    Some(vec![PortBinding {
                        host_ip: Some("0.0.0.0".into()),
                        host_port: Some("0".into()),
                    }]),
                )]
                .iter()
                .cloned()
                .collect(),
            ),
            memory: Some(512 * 1024 * 1024),
            pids_limit: Some(512),
            ..Default::default()
        }),
        ..Default::default()
    };
    let ContainerCreateResponse { id, .. } = docker
        .create_container(Some(CreateContainerOptions { name: cname, platform: None }), create_cfg)
        .await?;
    docker
        .start_container(&id, None::<StartContainerOptions<String>>)
        .await?;

    let inspect = docker.inspect_container(&id, None).await?;
    let port_map = inspect
        .network_settings
        .and_then(|n| n.ports)
        .ok_or_else(|| anyhow::anyhow!("missing port map"))?;
    let binding = port_map
        .get("2222/tcp")
        .and_then(|v| v.as_ref())
        .and_then(|v| v.first())
        .ok_or_else(|| anyhow::anyhow!("missing port binding"))?;
    let port = binding
        .host_port
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("missing host port"))?
        .parse::<u16>()?;

    let docker_cleanup = docker.clone();
    let id_cleanup = id.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5400)).await;
        let _ = docker_cleanup.stop_container(&id_cleanup, None).await;
        let _ = docker_cleanup.remove_container(&id_cleanup, None).await;
    });

    Ok(SshLaunch {
        host: "lab.example.com".into(),
        port,
        user: "student".into(),
        private_key_pem: priv_pem,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/launch", web::get().to(launch_handler)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
