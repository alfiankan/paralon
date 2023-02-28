use std::env;
use std::env::VarError;
use std::fs::File;
use std::io::Write;
use s3::error::S3Error;
use s3::{Bucket, Region};
use s3::creds::Credentials;

pub async fn pull_down_data_minio() -> Result<(), S3Error> {



    let bucket = Bucket::new(
        &*env::var("MINIO_BUCKET_NAME").unwrap(),
        Region::Custom {
            region: env::var("MINIO_REGION").unwrap(),
            endpoint: env::var("MINIO_ENDPOINT").unwrap(),
        },
        Credentials::new(
            Option::from(&*env::var("MINIO_ACCESS_KEY").unwrap()),
            Option::from(&*env::var("MINIO_SECRET_KEY").unwrap()),
            Option::from(&*env::var("MINIO_SECURITY_TOKEN").unwrap()),
            Option::from(&*env::var("MINIO_SESSION_TOKEN").unwrap()),
            Option::from(&*env::var("MINIO_PROFILE").unwrap()),
        )?,
    )?
        .with_path_style();

    let s3_path = &*env::var("MINIO_FILE_TARGET").unwrap();

    let response_data = bucket.get_object(s3_path).await?;
    assert_eq!(response_data.status_code(), 200);
    //assert_eq!(test, response_data.as_slice());
    //println!("{:?}", response_data.bytes());

    let path = &*env::var("MINIO_FILE_OUTPUT_TARGET").unwrap();

    let mut buffer = File::create(path).unwrap();

    buffer.write_all(response_data.bytes()).unwrap();

    Ok(())
}