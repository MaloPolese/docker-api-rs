mod common;
use clap::Clap;
use common::new_docker;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: Cmd,
}

#[derive(Clap)]
enum Cmd {
    Create {
        volume: String,
        #[clap(default_value = "overlay2")]
        driver: String,
    },
    Inspect {
        volume: String,
    },
    Delete {
        volume: String,
    },
    List,
    Prune,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let docker = new_docker()?;
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        Cmd::Create { volume, driver } => {
            use docker_api::api::VolumeCreateOpts;
            match docker
                .volumes()
                .create(
                    &VolumeCreateOpts::builder()
                        .name(volume)
                        .driver(driver)
                        .build(),
                )
                .await
            {
                Ok(info) => println!("{:?}", info),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Cmd::Inspect { volume } => {
            match docker.volumes().get(&volume).inspect().await {
                Ok(info) => println!("{:#?}", info),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::Delete { volume } => {
            match docker.volumes().get(&volume).delete().await {
                Ok(info) => println!("{:#?}", info),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::List => {
            match docker.volumes().list(&Default::default()).await {
                Ok(volumes) => {
                    for v in volumes.volumes {
                        println!("{:#?}", v)
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::Prune => {
            match docker.volumes().prune(&Default::default()).await {
                Ok(info) => println!("{:#?}", info),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
    }

    Ok(())
}
