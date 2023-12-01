use opendal::layers::LoggingLayer;
use opendal::services;
use opendal::Operator;
use opendal::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Pick a builder and configure it.
    let mut builder = services::S3::default();
    builder.bucket("hamsoft-test");
    builder.region("us-west-1");
    // Init an operator
    let op = Operator::new(builder)?
        // Init with logging layer enabled.
        .layer(LoggingLayer::default())
        .finish();

    // Write data
    // op.write("hello.txt", "Hello, World!").await?;

    // Read data
    let bs = op.read("hello.txt").await?;
    let payload: String = String::from_utf8(bs).unwrap();
    println!("{}", payload);
    
    let photo = op.read("ocr/input/portobello_caps.JPEG").await?;
    let payload: String = String::from_utf8(photo).unwrap();
    println!("{}", payload);
    // Delete
    // op.delete("hello.txt").await?;
    Ok(())
}