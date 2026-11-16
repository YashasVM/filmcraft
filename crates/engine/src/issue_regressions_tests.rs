//! Regression for source-derived sequence naming.
use std::sync::Arc;

use serde_json::json;

use crate::Session;

#[test]
fn issue_4844_sequence_uses_first_source_name_unless_explicit() {
    for mode in ["fromItem", "fromItems", "explicit", "empty"] {
        let mut s = Session::default();
        s.execute("file.openDemoProject", json!({})).unwrap();
        let source = s.active_sequence().unwrap().video_tracks[0].items[0].item;
        Arc::make_mut(&mut s.project).item_mut(source).unwrap().name = "Interview take 2.mp4".into();
        let count = s.project.sequences().count() + 1;
        let params = match mode {
            "fromItem" => json!({"fromItem": source.0}),
            "fromItems" => json!({"fromItems": [source.0]}),
            "explicit" => json!({"fromItem": source.0, "name": "My chosen name"}),
            _ => json!({}),
        };
        let result = s.execute("file.newSequence", params).unwrap();
        let id = filmcraft_project::ItemId(result["sequence"].as_u64().unwrap());
        let expected = match mode {
            "explicit" => "My chosen name".to_string(),
            "empty" => format!("Sequence {count:02}"),
            _ => "Interview take 2".to_string(),
        };
        assert_eq!(s.project.item(id).unwrap().name, expected, "{mode}");
    }
}
