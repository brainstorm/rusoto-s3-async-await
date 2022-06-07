use rusoto_s3::{ListObjectsV2Request, S3Client, S3, GetObjectRequest};
use rusoto_core::Region;
use futures::TryStreamExt;
use bytes::BytesMut;
use tokio;

pub async fn list_objs(client: S3Client, bucket: String, prefix: String) {
    let list_obj_req = ListObjectsV2Request {
        bucket,
        prefix: Some(prefix),
        ..ListObjectsV2Request::default()
    };

    println!("Request: {:?}", list_obj_req);

    let objects = client
        .list_objects_v2(list_obj_req)
        .await
        .unwrap()
        .contents
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>();

    println!("Result: {:?}", objects);
}

pub async fn bucket_obj_bytes(client: S3Client, bucket: String, _prefix: String, object: String) {
    let get_req = GetObjectRequest {
        bucket,
        key: object,
        ..Default::default()
    };

    let result = client
        .get_object(get_req)
        .await
        .expect("Couldn't GET object");

    let stream = result.body.unwrap();
    let body = stream.map_ok(|b| BytesMut::from(&b[..])).try_concat().await.unwrap();

    assert!(body.len() > 0);
    dbg!(body);
}

#[tokio::main]
async fn main() {
    let region = Region::default();
    let s3 = S3Client::new(region);
    let bucket= "umccr-misc-temp".to_string();
    let prefix = "htsget".to_string();
    let obj_test = "htsget/test.txt".to_string();
    let _obj_bai = "htsnexus_test_NA12878.bam.bai".to_string();
    let _obj_bam = "htsnexus_test_NA12878.bam".to_string();

    //let _objects = list_objs(s3, bucket, prefix).await;
    let _bytes = bucket_obj_bytes(s3, bucket, prefix, obj_test).await;
}
