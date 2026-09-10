//! `registerMap` / `parseGeoJSON` 的 Rust 表。geo 坐标系从这里取区域。

use std::sync::{Mutex, OnceLock};

use indexmap::IndexMap;

use crate::model::GeoRegion;
use crate::option::OptionValue;

#[derive(Debug, Clone)]
pub struct MapRecord {
    pub geo_json: OptionValue,
    pub special_areas: OptionValue,
}

fn registry() -> &'static Mutex<IndexMap<String, MapRecord>> {
    static REG: OnceLock<Mutex<IndexMap<String, MapRecord>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(IndexMap::new()))
}

pub fn register_map(name: String, geo_json: OptionValue, special_areas: OptionValue) {
    if let Ok(mut map) = registry().lock() {
        map.insert(
            name,
            MapRecord {
                geo_json,
                special_areas,
            },
        );
    }
}

pub fn get_map(name: &str) -> Option<MapRecord> {
    registry().lock().ok()?.get(name).cloned()
}

pub fn lookup_map_regions(name: &str) -> Vec<GeoRegion> {
    let rec = match get_map(name) {
        Some(r) => r,
        None => return Vec::new(),
    };
    parse_geojson(&rec.geo_json, "name")
}

pub fn parse_geojson(geo_json: &OptionValue, name_property: &str) -> Vec<GeoRegion> {
    let decoded = decode_utf8_geojson(geo_json);
    let features = match decoded.get("features").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return Vec::new(),
    };
    features
        .iter()
        .filter_map(|feat| feature_to_region(feat, name_property))
        .collect()
}

fn feature_to_region(feat: &OptionValue, name_property: &str) -> Option<GeoRegion> {
    let props = feat.get("properties")?;
    let name = props
        .get(name_property)
        .or_else(|| props.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let geom = feat.get("geometry")?;
    let gtype = geom.get("type").and_then(|v| v.as_str()).unwrap_or("");
    let coords = geom.get("coordinates")?;
    let mut polygons = Vec::new();
    match gtype {
        "Polygon" => {
            if let Some(outer) = ring_list(coords).into_iter().next() {
                polygons.push(outer);
            }
        }
        "MultiPolygon" => {
            if let Some(arr) = coords.as_array() {
                for poly in arr {
                    if let Some(outer) = ring_list(poly).into_iter().next() {
                        polygons.push(outer);
                    }
                }
            }
        }
        "LineString" => {
            let line = points_of(coords);
            if !line.is_empty() {
                polygons.push(line);
            }
        }
        "MultiLineString" => {
            if let Some(arr) = coords.as_array() {
                for line in arr {
                    let pts = points_of(line);
                    if !pts.is_empty() {
                        polygons.push(pts);
                    }
                }
            }
        }
        _ => {}
    }
    if polygons.is_empty() {
        return None;
    }
    let center = props
        .get("cp")
        .and_then(|v| v.as_array())
        .and_then(|a| Some((a.first()?.as_f64()?, a.get(1)?.as_f64()?)))
        .or_else(|| centroid(polygons.first()?));
    Some(GeoRegion {
        name,
        polygons,
        center,
    })
}

fn ring_list(coords: &OptionValue) -> Vec<Vec<(f64, f64)>> {
    coords
        .as_array()
        .map(|rings| rings.iter().map(points_of).filter(|r| !r.is_empty()).collect())
        .unwrap_or_default()
}

fn points_of(ring: &OptionValue) -> Vec<(f64, f64)> {
    ring.as_array()
        .map(|pts| {
            pts.iter()
                .filter_map(|p| {
                    let a = p.as_array()?;
                    Some((a.first()?.as_f64()?, a.get(1)?.as_f64()?))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn centroid(poly: &[(f64, f64)]) -> Option<(f64, f64)> {
    if poly.is_empty() {
        return None;
    }
    let n = poly.len() as f64;
    let x = poly.iter().map(|p| p.0).sum::<f64>() / n;
    let y = poly.iter().map(|p| p.1).sum::<f64>() / n;
    Some((x, y))
}

/// 官方压缩 GeoJSON（`UTF8Encoding`）先解码再当普通 FeatureCollection。
fn decode_utf8_geojson(geo_json: &OptionValue) -> OptionValue {
    let encoded = geo_json
        .get("UTF8Encoding")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if !encoded {
        return geo_json.clone();
    }
    let scale = geo_json
        .get("UTF8Scale")
        .and_then(|v| v.as_f64())
        .unwrap_or(1024.0);
    let mut out = geo_json.clone();
    let Some(features) = out.get_mut("features").and_then(|v| v.as_array_mut()) else {
        return out;
    };
    for feat in features.iter_mut() {
        let Some(geom) = feat.get_mut("geometry") else {
            continue;
        };
        let gtype = geom
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let offsets = geom.get("encodeOffsets").cloned();
        let coords = geom.get("coordinates").cloned();
        let (Some(offsets), Some(coords)) = (offsets, coords) else {
            continue;
        };
        let decoded = match gtype.as_str() {
            "LineString" => OptionValue::Array(decode_ring_value(&coords, &offsets, scale)),
            "Polygon" | "MultiLineString" => decode_rings_value(&coords, &offsets, scale),
            "MultiPolygon" => decode_multi_polygon(&coords, &offsets, scale),
            _ => coords,
        };
        if let Some(g) = feat.get_mut("geometry").and_then(|v| v.as_object_mut()) {
            g.insert("coordinates".into(), decoded);
            g.insert("UTF8Encoding".into(), OptionValue::Bool(false));
        }
    }
    if let Some(obj) = out.as_object_mut() {
        obj.insert("UTF8Encoding".into(), OptionValue::Bool(false));
    }
    out
}

fn decode_multi_polygon(coords: &OptionValue, offsets: &OptionValue, scale: f64) -> OptionValue {
    let Some(polys) = coords.as_array() else {
        return coords.clone();
    };
    let off_arr = offsets.as_array();
    OptionValue::Array(
        polys
            .iter()
            .enumerate()
            .map(|(i, poly)| {
                let off = off_arr.and_then(|a| a.get(i)).unwrap_or(offsets);
                decode_rings_value(poly, off, scale)
            })
            .collect(),
    )
}

fn decode_rings_value(coords: &OptionValue, offsets: &OptionValue, scale: f64) -> OptionValue {
    let Some(rings) = coords.as_array() else {
        return coords.clone();
    };
    let off_arr = offsets.as_array();
    OptionValue::Array(
        rings
            .iter()
            .enumerate()
            .map(|(i, ring)| {
                let off = off_arr.and_then(|a| a.get(i)).unwrap_or(offsets);
                OptionValue::Array(decode_ring_value(ring, off, scale))
            })
            .collect(),
    )
}

fn decode_ring_value(coord: &OptionValue, offsets: &OptionValue, scale: f64) -> Vec<OptionValue> {
    if let Some(s) = coord.as_str() {
        return decode_ring_str(s, offsets, scale)
            .into_iter()
            .map(|(x, y)| OptionValue::Array(vec![OptionValue::Number(x), OptionValue::Number(y)]))
            .collect();
    }
    coord.as_array().map(|a| a.to_vec()).unwrap_or_default()
}

fn decode_ring_str(coordinate: &str, offsets: &OptionValue, scale: f64) -> Vec<(f64, f64)> {
    let chars: Vec<char> = coordinate.chars().collect();
    let mut prev_x = offsets
        .as_array()
        .and_then(|a| a.first())
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let mut prev_y = offsets
        .as_array()
        .and_then(|a| a.get(1))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < chars.len() {
        let mut x = chars[i] as i32 - 64;
        let mut y = chars[i + 1] as i32 - 64;
        x = (x >> 1) ^ -(x & 1);
        y = (y >> 1) ^ -(y & 1);
        prev_x += x as f64;
        prev_y += y as f64;
        out.push((prev_x / scale, prev_y / scale));
        i += 2;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(pairs: Vec<(&str, OptionValue)>) -> OptionValue {
        let mut m = IndexMap::new();
        for (k, v) in pairs {
            m.insert(k.into(), v);
        }
        OptionValue::Object(m)
    }

    #[test]
    fn parse_plain_polygon() {
        let geo = obj(vec![(
            "features",
            OptionValue::Array(vec![obj(vec![
                (
                    "properties",
                    obj(vec![("name", OptionValue::String("A".into()))]),
                ),
                (
                    "geometry",
                    obj(vec![
                        ("type", OptionValue::String("Polygon".into())),
                        (
                            "coordinates",
                            OptionValue::Array(vec![OptionValue::Array(vec![
                                OptionValue::Array(vec![
                                    OptionValue::Number(0.0),
                                    OptionValue::Number(0.0),
                                ]),
                                OptionValue::Array(vec![
                                    OptionValue::Number(1.0),
                                    OptionValue::Number(0.0),
                                ]),
                                OptionValue::Array(vec![
                                    OptionValue::Number(1.0),
                                    OptionValue::Number(1.0),
                                ]),
                                OptionValue::Array(vec![
                                    OptionValue::Number(0.0),
                                    OptionValue::Number(0.0),
                                ]),
                            ])]),
                        ),
                    ]),
                ),
            ])]),
        )]);
        let regions = parse_geojson(&geo, "name");
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].name, "A");
        assert_eq!(regions[0].polygons[0].len(), 4);
    }
}
