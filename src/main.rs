use rusoto_s3::{ListObjectsV2Request, S3Client, S3};
use rusoto_core::Region;

pub async fn bucket_obj_bytes() {
    let region = Region::default();
    let s3 = S3Client::new(region);
    let bucket= "umccr-misc-temp".to_string();
    let prefix = "htsget".to_string();
    let _obj_bai = "htsnexus_test_NA12878.bam.bai".to_string();
    let _obj_bam = "htsnexus_test_NA12878.bam".to_string();

    let list_obj_req = ListObjectsV2Request {
        bucket: bucket.clone(),
        prefix: Some(prefix),
        ..ListObjectsV2Request::default()
    };

    println!("Request: {:?}", list_obj_req);

    let objects = s3
        .list_objects_v2(list_obj_req)
        .await
        .unwrap()
        .contents
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>();

    println!("Result: {:?}", objects);

//    let contents = objects.into_iter().map(move |object| {
//        let key = object.key.unwrap_or_default();
//        println!("{}", key);
//
//        s3.get_object(GetObjectRequest {
//            bucket: bucket.clone(),
//            key,
//            ..GetObjectRequest::default()
//        }).await
//            .map_err(Error::from)
//            .and_then(|obj| {
//                read_to_end(
//                    GzDecoder::new(obj.body.unwrap().into_async_read()),
//                    Vec::new(),
//                )
//                    .map(|(_, bytes)| Request::from_bytes(bytes).unwrap_or_default())
//                    .map_err(Error::from)
//            })
//    });
//    let mut rt = tokio::runtime::Runtime::new().unwrap();
//    let results = rt.block_on(future::join_all(contents));
//    for line in results
//        .unwrap_or_default()
//        .into_iter()
//        .flatten()
//        .collect::<Vec<_>>()
//    {
//        println!("{:#?}", line);
//    }

}

#[tokio::main]
async fn main() {
    let _bytes = bucket_obj_bytes().await;
}
