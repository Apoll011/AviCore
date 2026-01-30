use std::sync::Arc;

use content_resolver::{ContentSource, GitHubSource, ResourceResolver};

use crate::data::config::setting;

//TODO uncomment the checks
pub fn get_from_settings(settings_name: String) -> Option<ResourceResolver> {
    let setting = setting::<Vec<String>>(&settings_name)?;
    let mut resources: Vec<Arc<dyn ContentSource + 'static>> = vec![];

    for resolv in setting {
        let data = resolv.split(":").collect::<Vec<&str>>();
        //        if resolv.len() != 4 {
        //            continue;
        //        };

        if data.first()?.to_string().eq("git") {
            let repo_data = data.get(1)?.split("@").collect::<Vec<&str>>();

            //            if repo_data.len() != 2 {
            //                continue;
            //            };

            let s = Arc::new(GitHubSource::new(
                repo_data.first()?.to_string(),
                repo_data.get(1)?.to_string(),
                data.get(2)?.to_string(),
                data.get(3)?.to_string(),
            ));

            resources.push(s);
        };
    }

    let resolver = ResourceResolver::new(resources);

    Some(resolver)
}
