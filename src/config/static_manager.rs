use std::sync::{Arc, Mutex};
use anyhow::Error;
use super::{project_config::ProjectConfig, optimizely_config::OptimizelyConfig};

// StaticProjectConfigManager maintains a static copy of the project config
#[derive(Clone, Debug, Default)]
pub struct StaticProjectConfigManager {
	project_config: Arc<Mutex<dyn ProjectConfig>>,
	optimizely_config: Arc<Mutex<OptimizelyConfig>>
}

impl StaticProjectConfigManager {
    // new creates a new instance of the manager with the given project config
    pub fn new(config: Box<dyn ProjectConfig>) -> Self {
        return Self {
            project_config: Arc::new(Mutex::new(config.into())),
            ..Default::default()
        }
    }
    // new_static_project_config_manager_with_options creates a new instance of the manager with the given sdk key and some options
    pub fn new_with_options(sdk_key: String, config_manger_options: Vec<String>) -> Self {

        // staticProjectConfigManager := newConfigManager(sdkKey, configMangerOptions...)
        // if sdkKey != "" {
        //     staticProjectConfigManager.SyncConfig()
        // } else if len(staticProjectConfigManager.initDatafile) > 0 {
        //     staticProjectConfigManager.setInitialDatafile(staticProjectConfigManager.initDatafile)
        // }
        // projectConfig, err := staticProjectConfigManager.GetConfig()
        // if err != nil {
        //     logger.Error("unable to get project config, error returned:", err)
        //     return nil
        // }

        // return &StaticProjectConfigManager{
        //     projectConfig: projectConfig,
        // }
        Self {
            ..Default::default()
        }

    }

    // get_config returns the project config
    pub fn get_config(&self) -> Result<dyn ProjectConfig, Error> {
        // cm.configLock.Lock()
        // defer cm.configLock.Unlock()
        // return cm.projectConfig, nil
        Ok(self.project_config)
    }

    // get_optimizely_config returns the optimizely project config
    pub fn get_optimizely_config(&self) -> Option<OptimizelyConfig> {
        // cm.configLock.Lock()
        // defer cm.configLock.Unlock()
        // if cm.optimizelyConfig != nil {
        //     return cm.optimizelyConfig
        // }
        // optimizelyConfig := NewOptimizelyConfig(cm.projectConfig)
        // cm.optimizelyConfig = optimizelyConfig

        Some(self.optimizely_config)
    }

    // remove_on_project_config_update here satisfies interface
    pub fn remove_on_project_config_update(id: i64) -> Result<(), Error> {
        // return errors.New("method RemoveOnProjectConfigUpdate does not have any effect on StaticProjectConfigManager")
        Ok(())
    }

    // on_project_config_update here satisfies interface
    pub fn on_project_config_update(callback: fn(ProjectConfigUpdateNotification)) -> Result<i32, Error> {
        // return 0, errors.New("method OnProjectConfigUpdate does not have any effect on StaticProjectConfigManager")
        Ok(0)
    }
}