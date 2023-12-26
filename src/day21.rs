use crate::util::MyError;
use crate::AppState;
use axum::extract::State;
use axum::{debug_handler, extract::Path};
use dms_coordinates::DMS3d;
use s2::cellid::CellID;
use s2::latlng::LatLng;
use s2::point::Point;
use serde::Deserialize;
use serde_json::Value;

#[debug_handler]
pub async fn s2_to_dms(Path(data): Path<String>) -> String {
    let num = u64::from_str_radix(&data, 2).unwrap();

    let center = Point::from(CellID(num));

    let pos = LatLng::from(center);

    let dms = DMS3d::from_ddeg_angles(pos.lat.deg(), pos.lng.deg(), None);

    let lat = dms.latitude;
    let dir = if pos.lat.deg() > 0. { 'N' } else { 'S' };
    let result1 = format!("{}°{}'{:.3}''{dir}", lat.degrees, lat.minutes, lat.seconds);

    let lng = dms.longitude;
    let dir = if pos.lng.deg() > 0. { 'E' } else { 'W' };
    let result2 = format!("{}°{}'{:.3}''{dir}", lng.degrees, lng.minutes, lng.seconds);

    format!("{result1} {result2}")
}

#[derive(Debug, Deserialize)]
struct APIResponse {
    results: Vec<Value>,
}

#[debug_handler]
pub async fn country_lookup(
    Path(data): Path<String>,
    State(AppState { secrets, .. }): State<AppState>,
) -> Result<String, MyError> {
    let num = u64::from_str_radix(&data, 2).unwrap();

    let center = Point::from(CellID(num));

    let pos = LatLng::from(center);

    let lat = pos.lat.deg();
    let lng = pos.lng.deg();

    let opencage_key = if let Some(secret) = secrets {
        secret
    } else {
        return Err(MyError::CustomError("Secret was not Found".to_string()));
    };

    let url = format!(
        "https://api.opencagedata.com/geocode/v1/json?q={lat}%2C{lng}&key={opencage_key}&pretty=1"
    );

    match reqwest::get(url).await {
        Ok(result) => match result.json::<APIResponse>().await {
            Ok(res) => {
                let res = res.results[0]
                    .get("components")
                    .unwrap()
                    .get("country")
                    .unwrap().as_str().unwrap();
                Ok(res.to_string())
            }
            Err(err) => Err(MyError::CustomError(err.to_string())),
        },
        Err(err) => Err(MyError::CustomError(err.to_string())),
    }
}
