pub mod structs;
pub mod processes;

use structs::FootPrint;
use processes::{merge_footprint_with_events, read_footprint};


pub fn merge_event_ids_with_footprint(event_ids: Vec<i32>,
       base_path: String) -> Vec<FootPrint> {
    let foot_prints = read_footprint(base_path).unwrap();
    return merge_footprint_with_events(event_ids, foot_prints)
}
