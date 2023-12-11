use crate::util::MyError;
use axum::{debug_handler, extract::RawPathParams, response::Json};

#[debug_handler]
pub async fn cube_bits(parmas: RawPathParams) -> Result<Json<i32>, MyError> {
    let nums = parmas.iter().next().unwrap().1;

    let nums = nums
        .split_terminator('/')
        .map(|v| v.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>();

    if let Ok(nums) = nums {
        if nums.len() > 20 {
            return Err(MyError::SledRangeExceed);
        }

        let result = nums.iter().fold(0, |acc, x| acc ^ x);
        let result = result.pow(3);
        Ok(Json(result))
    } else {
        Err(MyError::ParseError)
    }
}
