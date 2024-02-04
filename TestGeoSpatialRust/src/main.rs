use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use csv::ReaderBuilder;
use geo::Contains;
use geojson::{GeoJson, Geometry, Value};
use kiddo::float::{distance::SquaredEuclidean, kdtree::KdTree};
use kiddo::NearestNeighbour;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub const EARTH_RADIUS_IN_KM: f64 = 6371.0;

#[derive(Debug, Deserialize, Clone)]
pub struct CsvRecord {
    id: usize,
    latitude: f64,
    longitude: f64,
}

impl CsvRecord {
    pub fn as_xyz(&self) -> [f64; 3] {
        degrees_lat_lng_to_unit_sphere(self.latitude, self.longitude)
    }
}

pub fn degrees_lat_lng_to_unit_sphere(lat: f64, lng: f64) -> [f64; 3] {
    // convert from degrees to radians
    let lat = lat.to_radians();
    let lng = lng.to_radians();

    // convert from ra/dec to xyz coords on unit sphere
    [lat.cos() * lng.cos(), lat.cos() * lng.sin(), lat.sin()]
}

pub fn kilometres_to_unit_sphere_squared_euclidean(km_dist: f64) -> f64 {
    (km_dist / EARTH_RADIUS_IN_KM).powi(2)
}

pub fn read_random_locations_csv(file_path: &str) -> Vec<CsvRecord> {
    let mut cities: Vec<CsvRecord> = vec![];
    let mut reader = ReaderBuilder::new().from_path(file_path).unwrap();
    //let mut reader = Reader::from_path(file_path).unwrap();
    for result in reader.deserialize() {
        if let Ok(city_csv_record) = result {
            cities.push(city_csv_record);
        } else {
            println!("Couldnt read csv record")
        }
    }

    cities
}

pub struct RegionTrees {
    pub regions: Vec<geo::Polygon>,
    pub trees_map: HashMap<usize, KdTree<f64, usize, 3, 32, u32>>,
    pub records: Vec<CsvRecord>
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    longitude: f64,
    latitude: f64,
}
#[derive(Debug, Serialize)]
pub struct SearchResult {
    id: usize,
    longitude: f64,
    latitude: f64,
}

pub async fn search(
    Query(params): Query<SearchRequest>,
    State(app_sate): State<Arc<RegionTrees>>,
) -> Result<Json<Vec<SearchResult>>, (StatusCode, String)> {
    //let start = Instant::now();

    let mut found_ids: Vec<NearestNeighbour<f64, usize>> = vec![];
    for (index, region) in app_sate.regions.iter().enumerate() {
        if region.contains(&geo::Point::new(params.longitude, params.latitude)) {
            if let Some(kd_tree) = app_sate.trees_map.get(&index) {
                let query = degrees_lat_lng_to_unit_sphere(params.latitude, params.longitude);
                let dist = kilometres_to_unit_sphere_squared_euclidean(40.0);
                found_ids = kd_tree.within::<SquaredEuclidean>(&query, dist);
            } else {
                break;
            }
        }
    }
    let mut found_records: Vec<SearchResult> = vec![];
    for found_id in found_ids {
        let r = app_sate.records.get(found_id.item).unwrap().clone();
        found_records.push(SearchResult {id: r.id, longitude: r.longitude, latitude: r.latitude})
    }

    //println!("Search finished in: {:?}", start.elapsed());

    Ok(Json(found_records))
}

#[tokio::main]
async fn main() {
    let regions = read_france_geojson_files();
    let mut region_to_tree: HashMap<usize, KdTree<f64, usize, 3, 32, u32>> = HashMap::new();
    for (index, _region) in regions.iter().enumerate() {
        let tree: KdTree<f64, usize, 3, 32, u32> = KdTree::new();
        region_to_tree.insert(index, tree);
    }

    println!("Reading locations and creating trees");
    let records = read_random_locations_csv("/home/miladamery/Geo/france_ranom_coordinates.csv");
    for record in &records {
        for (index, region) in regions.iter().enumerate() {
            if region.contains(&geo::Point::new(record.longitude, record.latitude)) {
                region_to_tree.get_mut(&index).unwrap().add(&record.as_xyz(), record.id.clone());
            }
        }
    }
    println!("Reading locations and Creating trees ended");


    let region_trees = RegionTrees { regions, trees_map: region_to_tree, records };
    let app = Router::new()
        .route("/", get(search))
        .with_state(Arc::new(region_trees));

    println!("Starting server .....");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8085").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Server started.")
}

fn read_france_geojson_files() -> Vec<geo::Polygon> {
    let base_path = "/home/miladamery/Geo/buffered-france-regions-geojson";
    let files = vec![
        "Buffered-Auvergne-Rhone-Alpes.geojson",
        "Buffered-Bourgogne-Franche-Comte.geojson",
        "Buffered-Bretagne.geojson",
        "Buffered-Centre-Val-de-Loire.geojson",
        "Buffered-Corse.geojson",
        "Buffered-Grand-Est.geojson",
        "Buffered-Hauts-de-France.geojson",
        "Buffered-ile-de-France.geojson",
        "Buffered-Normandie.geojson",
        "Buffered-Nouvelle-Aquitaine.geojson",
        "Buffered-Occitanie.geojson",
        "Buffered-Pays-de-la-Loire.geojson",
        "Buffered-Provence-Alpes-Cote-dAzur.geojson",
    ];

    let mut regions: Vec<geo::Polygon> = vec![];
    for file in files {
        let path = format!("{}/{}", base_path, file);
        let geo_json_str = fs::read_to_string(path)
            .expect(format!("Couldn't read file with name : {:?}", file).as_str());
        let geo_json: GeoJson = geo_json_str.parse::<GeoJson>().unwrap();
        match geo_json {
            GeoJson::Geometry(a) => { match_geometry(&a, &mut regions) }
            GeoJson::Feature(a) => { match_geometry(&a.geometry.unwrap(), &mut regions) }
            GeoJson::FeatureCollection(a) => {
                for feature in a {
                    match_geometry(&feature.geometry.unwrap(), &mut regions)
                }
            }
        }
    }

    regions
}

/// Process GeoJSON geometries
fn match_geometry(geom: &Geometry, regions: &mut Vec<geo::Polygon>) {
    match geom.clone().value {
        Value::Polygon(_) => {
            let region: geo::Polygon = geom.clone().value.try_into().unwrap();
            regions.push(region);
        }
        Value::MultiPolygon(_) => {
            let region: geo::MultiPolygon = geom.clone().try_into().unwrap();
            for sub_region in region {
                regions.push(sub_region);
            }
        }
        Value::GeometryCollection(ref gc) => {
            // !!! GeometryCollections contain other Geometry types, and can
            // nest — we deal with this by recursively processing each geometry
            for geometry in gc {
                match_geometry(geometry, regions);
            }
        }
        // Point, LineString, and their Multi– counterparts
        _ => println!("Matched some other geometry"),
    }
}
