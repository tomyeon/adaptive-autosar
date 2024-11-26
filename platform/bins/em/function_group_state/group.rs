use anyhow::Result;
use ara_exec::manifest::execution_manifest::ExecutionManifest;
use ara_exec::manifest::machine_manifest::MachineManifest;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
enum GroupingError {
    #[error("Not in the same mode for {0}")]
    NotInTheSameMode(String),
    #[error("Not dependency application : {0} for {1}")]
    NoDepdencyApps(String, String),
}

// grouping manifest based on dependency

pub type FunctionGroupStateHashMap = HashMap<String, Vec<ExecutionManifest>>;
pub type FunctionGroupHashMap = HashMap<String, FunctionGroupStateHashMap>;

/*pub struct InternalFgMode {
    pub function_group: String,
    pub state: String,
}*/

// TBD : MachineFg도 Off를 넣어야 한다.
// grouping manifest base on function group state
pub fn group(
    machine_manifest: MachineManifest,
    execution_manifests: Vec<ExecutionManifest>,
) -> Result<FunctionGroupHashMap> {
    let mut function_group = FunctionGroupHashMap::new();

    // MachineFG
    //  |- Startup
    //  |       |- "App a"
    //  |       |- "App b"
    //  |       \- "App c"
    //  |- Restart
    //  |- Shutfown
    //  \- Off

    for (fg_name, fg_states) in &machine_manifest.function_group_set {
        let mut mode = HashMap::new();
        for fg_state in &fg_states.mode {
            mode.insert(fg_state.clone(), Vec::new());
        }
        function_group.insert(fg_name.clone(), mode);
    }

    // collect all manifest
    for manifest in &execution_manifests {
        for dependency in &manifest.mode_dependency {
            let (group_name, mode_name) = dependency.split_once('.').unwrap();
            let group_name = group_name.to_owned();
            let mode_name = mode_name.to_owned();

            let mode = function_group.get_mut(&group_name).unwrap();
            let manifest_list = mode.get_mut(&mode_name).unwrap();
            manifest_list.push(manifest.clone());
        }
    }

    // prioritize by app dependenies

    for (_, mode) in &mut function_group {
        for (mode_name, manifest_list) in mode {
            let mut index = 0;
            for _ in index..manifest_list.len() {
                let app_name = &manifest_list[index].name;
                if !manifest_list[index].app_dependency.is_empty() {
                    let mut depend_index = None;
                    for dependency in &manifest_list[index].app_dependency {
                        let mut next_index = index + 1;
                        let (depend_app, _) = dependency.split_once('.').unwrap();
                        for _ in next_index..manifest_list.len() {
                            if depend_app == manifest_list[next_index].name {
                                // depdency application should in the same state
                                let depend_mode = format!("{}.", mode_name);
                                if !manifest_list[next_index]
                                    .mode_dependency
                                    .iter()
                                    .any(|s| s.contains(&depend_mode))
                                {
                                    return Err(
                                        GroupingError::NotInTheSameMode(app_name.clone()).into()
                                    );
                                }
                                depend_index = Some(next_index);
                            }
                            next_index += 1;
                        }
                    }

                    if let Some(di) = depend_index {
                        // move `index` value next to `di` poistion
                        let this_manifest = manifest_list.remove(index);
                        manifest_list.insert(di, this_manifest);
                        continue;
                    } else {
                        let dependency_apps = manifest_list[index].app_dependency.join(",");
                        return Err(GroupingError::NoDepdencyApps(
                            app_name.clone(),
                            dependency_apps,
                        )
                        .into());
                    }
                }

                index += 1;
            }

            if mode_name == "Off" {
                // FIXME : to const variable
                // reverse
                manifest_list.reverse();
            }
        }
    }

    Ok(function_group)
}

// "On" State
// -------------------------------------------------------------------------------
//  "A" -> "B" -> "C"       | "C" -> "B" -> "A"
// -------------------------------------------------------------------------------
//  "A" -> "C"              | "C" -> "B" | "A"
//  "B" -> "C"              |
// -------------------------------------------------------------------------------
//  "A" -> "B" -> "C"       | "C" ->  "B" | "D"   -> "E" | "A"    -> "P"
//  "D" -> "C"              |
//  "E" -> "B"              |
//  "P" -> "A"              |

#[cfg(test)]
mod tests {

    #[test]
    fn grouping_test() {}
}
