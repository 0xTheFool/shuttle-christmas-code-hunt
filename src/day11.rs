use crate::util::MyError;
use axum::debug_handler;
use axum::extract::Multipart;

#[debug_handler]
pub async fn get_no_of_red_pixels(mut multipart: Multipart) -> Result<String, MyError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "image" {
            let data = field.bytes().await.unwrap();

            let image = image::load_from_memory(&data[..]).unwrap().into_rgb8();

            let count = image.pixels().fold(0, |acc, pix| {
                let pix = pix.0;
                if pix[0] as u32 > pix[1] as u32 + pix[2] as u32 {
                    acc + 1
                } else {
                    acc
                }
            });

            return Ok(format!("{count}"));
        }
    }

    Err(MyError::CustomError("No Image Provided".to_string()))
}
