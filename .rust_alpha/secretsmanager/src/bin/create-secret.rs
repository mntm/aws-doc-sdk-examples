/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_sdk_secretsmanager::{Client, Config, Error, Region, PKG_VERSION};
use aws_types::region;
use aws_types::region::ProvideRegion;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the secret.
    #[structopt(short, long)]
    name: String,

    /// The value of the secret.
    #[structopt(short, long)]
    secret_value: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

/// Creates a secret.
/// # Arguments
///
/// * `-n NAME` - The name of the secret.
/// * `-s SECRET_VALUE` - The secret value.
/// * `[-r REGION]` - The Region in which the client is created.
///    If not supplied, uses the value of the **AWS_REGION** environment variable.
///    If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt {
        name,
        region,
        secret_value,
        verbose,
    } = Opt::from_args();

    let region = region::ChainProvider::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    println!();

    if verbose {
        println!("SecretsManager client version: {}", PKG_VERSION);
        println!(
            "Region:                        {}",
            region.region().unwrap().as_ref()
        );
        println!("Secret name:                   {}", &name);
        println!("Secret value:                  {}", &secret_value);
        println!();
    }

    let config = Config::builder().region(region).build();
    let client = Client::from_conf(config);

    client
        .create_secret()
        .name(name)
        .secret_string(secret_value)
        .send()
        .await?;

    println!("Created secret");

    Ok(())
}
