// Copyright 2017-2018 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

extern crate navitia_model;
extern crate tempdir;

use navitia_model::gtfs;
use tempdir::TempDir;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn load_minimal_agency() {
    let agency_content = "agency_name,agency_url,agency_timezone\n
    My agency,http://my-agency_url.com,Europe/London";
    let tmp_dir = TempDir::new("navitia_model_tests").expect("create temp dir");
    let file_path = tmp_dir.path().join("agency.txt");
    let mut f = File::create(&file_path).unwrap();
    f.write_all(agency_content.as_bytes()).unwrap();

    let (networks, companies) = gtfs::read_agency(tmp_dir.path());
    tmp_dir.close().expect("delete temp dir");
    assert_eq!(1, networks.len());
    let agency = networks.iter().next().unwrap().1;
    assert_eq!("default_agency_id", agency.id);
    assert_eq!(1, companies.len());
}

#[test]
fn load_standard_agency() {
    let agency_content = "agency_id,agency_name,agency_url,agency_timezone\n
id_1,My agency,http://my-agency_url.com,Europe/London";
    let tmp_dir = TempDir::new("navitia_model_tests").expect("create temp dir");
    let file_path = tmp_dir.path().join("agency.txt");
    let mut f = File::create(&file_path).unwrap();
    f.write_all(agency_content.as_bytes()).unwrap();

    let (networks, companies) = gtfs::read_agency(tmp_dir.path());
    tmp_dir.close().expect("delete temp dir");
    assert_eq!(1, networks.len());
    assert_eq!(1, companies.len());
}

#[test]
fn load_complete_agency() {
    let agency_content = "agency_id,agency_name,agency_url,agency_timezone,agency_lang,agency_phone,agency_fare_url,agency_email\n
id_1,My agency,http://my-agency_url.com,Europe/London,EN,0123456789,http://my-agency_fare_url.com,my-mail@example.com";
    let tmp_dir = TempDir::new("navitia_model_tests").expect("create temp dir");
    let file_path = tmp_dir.path().join("agency.txt");
    let mut f = File::create(&file_path).unwrap();
    f.write_all(agency_content.as_bytes()).unwrap();

    let (networks, companies) = gtfs::read_agency(tmp_dir.path());
    tmp_dir.close().expect("delete temp dir");
    assert_eq!(1, networks.len());
    let network = networks.iter().next().unwrap().1;
    assert_eq!("id_1", network.id);
    assert_eq!(1, companies.len());
}

#[test]
#[should_panic]
fn load_2_agencies_with_no_id() {
    let agency_content = "agency_name,agency_url,agency_timezone\n
My agency 1,http://my-agency_url.com,Europe/London
My agency 2,http://my-agency_url.com,Europe/London";
    let tmp_dir = TempDir::new("navitia_model_tests").expect("create temp dir");
    let file_path = tmp_dir.path().join("agency.txt");
    let mut f = File::create(&file_path).unwrap();
    f.write_all(agency_content.as_bytes()).unwrap();
    gtfs::read_agency(tmp_dir.path());
    tmp_dir.close().expect("delete temp dir");
}

#[test]
fn load_one_stop_point() {
    let stops_content = "stop_id,stop_name,stop_lat,stop_lon\n\
                         id1,my stop name,0.1,1.2";
    let tmp_dir = TempDir::new("navitia_model_tests").expect("create temp dir");
    let file_path = tmp_dir.path().join("stops.txt");
    let mut f = File::create(&file_path).unwrap();
    f.write_all(stops_content.as_bytes()).unwrap();

    let (stop_areas, stop_points) = gtfs::read_stops(tmp_dir.path());
    tmp_dir.close().expect("delete temp dir");
    assert_eq!(1, stop_areas.len());
    assert_eq!(1, stop_points.len());
    let stop_area = stop_areas.iter().next().unwrap().1;
    assert_eq!("Navitia:id1", stop_area.id);

    assert_eq!(1, stop_points.len());
    let stop_point = stop_points.iter().next().unwrap().1;
    assert_eq!("Navitia:id1", stop_point.stop_area_id);
}

#[test]
fn stop_code_on_stops() {
    let stops_content =
        "stop_id,stop_code,stop_name,stop_lat,stop_lon,location_type,parent_station\n\
         stoppoint_id,1234,my stop name,0.1,1.2,0,stop_area_id\n\
         stoparea_id,5678,stop area name,0.1,1.2,1,";
    let tmp_dir = TempDir::new("navitia_model_tests").expect("create temp dir");
    let file_path = tmp_dir.path().join("stops.txt");
    let mut f = File::create(&file_path).unwrap();
    f.write_all(stops_content.as_bytes()).unwrap();

    let (stop_areas, stop_points) = gtfs::read_stops(tmp_dir.path());
    tmp_dir.close().expect("delete temp dir");
    //validate stop_point code
    assert_eq!(1, stop_points.len());
    let stop_point = stop_points.iter().next().unwrap().1;
    assert_eq!(1, stop_point.codes.len());
    let code = stop_point.codes.iter().next().unwrap();
    assert_eq!(code.0, "gtfs_stop_code");
    assert_eq!(code.1, "1234");

    //validate stop_area code
    assert_eq!(1, stop_areas.len());
    let stop_area = stop_areas.iter().next().unwrap().1;
    assert_eq!(1, stop_area.codes.len());
    let code = stop_area.codes.iter().next().unwrap();
    assert_eq!(code.0, "gtfs_stop_code");
    assert_eq!(code.1, "5678");
}

#[test]
fn no_stop_code_on_autogenerated_stoparea() {
    let stops_content =
        "stop_id,stop_code,stop_name,stop_lat,stop_lon,location_type,parent_station\n\
         stoppoint_id,1234,my stop name,0.1,1.2,0,";
    let tmp_dir = TempDir::new("navitia_model_tests").expect("create temp dir");
    let file_path = tmp_dir.path().join("stops.txt");
    let mut f = File::create(&file_path).unwrap();
    f.write_all(stops_content.as_bytes()).unwrap();

    let (stop_areas, _) = gtfs::read_stops(tmp_dir.path());
    tmp_dir.close().expect("delete temp dir");
    //validate stop_area code
    assert_eq!(1, stop_areas.len());
    let stop_area = stop_areas.iter().next().unwrap().1;
    assert_eq!(0, stop_area.codes.len());
}
