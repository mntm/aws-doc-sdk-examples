/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_sdk_s3::{Client, Config, Error, Region, PKG_VERSION};
use aws_types::region;
use aws_types::region::ProvideRegion;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// Whether to only get buckets in the Region.
    #[structopt(short, long)]
    strict: bool,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

/// Lists your Amazon S3 buckets, or just the buckets in the Region.
/// # Arguments
///
/// * `[-s]` - Only list bucket in the Region.
/// * `[-r REGION]` - The Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt {
        region,
        strict,
        verbose,
    } = Opt::from_args();

    let region = region::ChainProvider::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    println!();

    let region_str: String = String::from(region.region().unwrap().as_ref());

    if verbose {
        println!("S3 client version: {}", PKG_VERSION);
        println!("Region:            {}", region.region().unwrap().as_ref());

        if strict {
            println!("Only lists buckets in the Region.");
        } else {
            println!("Lists all buckets.");
        }

        println!();
    }

    let conf = Config::builder().region(region).build();
    let client = Client::from_conf(conf);

    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets.unwrap_or_default();
    let num_buckets = buckets.len();

    let mut in_region = 0;

    for bucket in &buckets {
        if strict {
            let r = client
                .get_bucket_location()
                .bucket(bucket.name.as_deref().unwrap_or_default())
                .send()
                .await?;

            if r.location_constraint.unwrap().as_ref() == region_str {
                println!("{}", bucket.name.as_deref().unwrap_or_default());
                in_region += 1;
            }
        } else {
            println!("{}", bucket.name.as_deref().unwrap_or_default());
        }
    }

    println!();
    if strict {
        println!(
            "Found {} buckets in the {} region out of a total of {} buckets.",
            in_region, region_str, num_buckets
        );
    } else {
        println!("Found {} buckets in all regions.", num_buckets);
    }

    Ok(())
}
