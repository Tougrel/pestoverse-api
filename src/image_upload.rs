use image::{codecs::jpeg::JpegEncoder, imageops, DynamicImage};
use nanoid::nanoid;
use worker::{
    console_log, Bucket, FormData, FormEntry, HttpMetadata, Request, Response, RouteContext,
};

pub async fn handle(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let form_data = req.form_data().await?;
    let raw_image = get_file(&form_data).await?;
    let name = get_name(&form_data).await?;

    let shrunk = resize_image(&raw_image, 800);
    let placeholder = resize_image(&raw_image, 20);

    let bucket = ctx.bucket("IMAGES")?;
    let identifier = nanoid!(10);
    let key = format!("{}-{}", name, identifier);

    upload(&bucket, format!("resized/{}.jpg", key), &shrunk).await?;
    upload(&bucket, format!("placeholder/{}.jpg", key), &placeholder).await?;
    upload(&bucket, format!("full/{}.jpg", key), &raw_image).await?;

    Response::ok(key)
}

async fn get_name(form_data: &FormData) -> worker::Result<String> {
    let mut name = "unknown".to_string();
    if let Some(entry) = form_data.get("name") {
        match entry {
            FormEntry::File(_) => return Err("invalid name".into()),
            FormEntry::Field(value) => name = value,
        }
    }
    Ok(name)
}

async fn get_file(form_data: &FormData) -> worker::Result<DynamicImage> {
    let mut raw_image: Option<DynamicImage> = None;
    if let Some(entry) = form_data.get("file") {
        match entry {
            FormEntry::File(file) => {
                let bytes = file.bytes().await?;
                raw_image = Some(image::load_from_memory(&bytes).expect("Couldn't load image"));
            }
            FormEntry::Field(_) => return Err("No file uploaded".into()),
        }
    }

    if raw_image.is_none() {
        return Err("Please provide a file".into());
    }

    let raw_image = raw_image.unwrap();
    Ok(raw_image)
}

fn get_resized_dimensions(current_width: u32, current_height: u32, max_width: u32) -> (u32, u32) {
    let new_width = max_width;

    console_log!("{} {}", current_width, current_height);

    let ratio = new_width as f32 / current_width as f32;
    console_log!("ratio: {:?}", ratio);
    let new_height = (current_height as f32 * ratio) as u32;

    (new_width, new_height)
}

fn resize_image(image: &DynamicImage, max_width: u32) -> DynamicImage {
    let (width, height) = get_resized_dimensions(image.width(), image.height(), max_width);
    let filter = imageops::FilterType::CatmullRom;
    image.resize(width, height, filter)
}

fn encode_image(image: &DynamicImage) -> Vec<u8> {
    let pixels = image.as_bytes();
    let mut data: Vec<u8> = Vec::new();
    let mut encoder = JpegEncoder::new(&mut data);
    let _result = encoder.encode(pixels, image.width(), image.height(), image.color());

    data
}

async fn upload(bucket: &Bucket, key: String, image: &DynamicImage) -> worker::Result<()> {
    let data = encode_image(image);
    let metadata = HttpMetadata {
        content_type: Some("image/jpeg".to_string()),
        ..HttpMetadata::default()
    };
    let _res = bucket
        .put(key, data)
        .http_metadata(metadata)
        .execute()
        .await?;

    Ok(())
}
