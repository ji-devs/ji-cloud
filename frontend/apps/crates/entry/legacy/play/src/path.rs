use utils::path::media_url;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn manifest_url(jig_id:&str) -> String {
            media_url(&format!("legacy/examples/{}/ji/manifest.json", jig_id))
        }

        pub fn module_url(jig_id:&str, module_index:usize) -> String {
            media_url(&format!("legacy/examples/{}/ji/module-{}.json", jig_id, module_index+1))
        }

    } else {
    }
}
