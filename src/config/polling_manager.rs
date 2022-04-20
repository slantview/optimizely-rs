use std::{time::{Instant, Duration}, sync::{Mutex, Arc}};
use anyhow::{Error, Ok};
use log::info;

use crate::notification::center::Center;

use super::{project_config::ProjectConfig, optimizely_config::OptimizelyConfig};

/// DEFAULT_POLLING_INTERVAL sets default interval for polling manager
const DEFAULT_POLLING_INTERVAL: Duration = Duration::from_secs(5*60); // default to 5 minutes for polling
/// MODIFIED_SINCE header key for request
const MODIFIED_SINCE: &str = "If-Modified-Since";
/// LAST_MODIFIED header key for response
const LAST_MODIFIED: &str = "Last-Modified";
/// DATAFILE_URL_TEMPLATE is used to construct the endpoint for retrieving regular datafile from the CDN
const DATAFILE_URL_TEMPLATE: &str = "https://cdn.optimizely.com/datafiles/%s.json";
/// AUTH_DATAFILE_URL_TEMPLATE is used to construct the endpoint for retrieving authenticated datafile from the CDN
const AUTH_DATAFILE_URL_TEMPLATE: &str = "https://config.optimizely.com/datafiles/auth/%s.json";
/// ERR_403_FORBIDDEN is 403Forbidden specific error
const ERR_403_FORBIDDEN: Error = Error::msg("unable to fetch fresh datafile (consider rechecking SDK key), status code: 403 Forbidden");

// PollingProjectConfigManager maintains a dynamic copy of the project config by continuously polling for the datafile
// from the Optimizely CDN at a given (configurable) interval.
#[derive(Clone, Debug, Default)]
pub struct PollingProjectConfigManager {
	pub datafile_url_template: String,
	pub init_datafile: Vec<u8>,
	pub last_modified: String,
	pub notification_center: Box<dyn Center>,
	pub polling_interval: Duration,
	pub requester: Requester,
	pub sdk_key: String,
	pub datafile_access_token: String,
	pub project_config:    Arc<Mutex<Box<dyn ProjectConfig>>>,
	pub optimizely_config: Option<OptimizelyConfig>
}

// Optionpub fn is used to provide custom configuration to the PollingProjectConfigManager.
// struct Optionpub fn = |ppm: Option<PollingProjectConfigManager>| {};

impl PollingProjectConfigManager {

    pub fn new_config_manager(sdk_key: String, config_options: Vec<String>) -> Self {
        // pollingProjectConfigManager := PollingProjectConfigManager{
        //     notificationCenter: registry.GetNotificationCenter(sdkKey),
        //     pollingInterval:    DefaultPollingInterval,
        //     requester:          utils.NewHTTPRequester(),
        //     sdkKey:             sdkKey,
        // }

        // for _, opt := range configOptions {
        //     opt(&pollingProjectConfigManager)
        // }

        // if pollingProjectConfigManager.datafileURLTemplate == "" {
        //     if pollingProjectConfigManager.datafileAccessToken != "" {
        //         pollingProjectConfigManager.datafileURLTemplate = AuthDatafileURLTemplate
        //     } else {
        //         pollingProjectConfigManager.datafileURLTemplate = DatafileURLTemplate
        //     }
        // }
        // pollingProjectConfigManager.setAuthHeaderIfDatafileAccessTokenPresent()
        // return &pollingProjectConfigManager
        Self {
            ..PollingProjectConfigManager::default()
        }
    }

    // new_polling_project_config_manager returns an instance of the polling config manager with the customized configuration
    pub fn new_polling_project_config_manager(sdk_key: String, polling_manger_options: Vec<String>) -> Self {

        // pollingProjectConfigManager := newConfigManager(sdkKey, pollingMangerOptions...)

        // if len(pollingProjectConfigManager.initDatafile) > 0 {
        //     pollingProjectConfigManager.setInitialDatafile(pollingProjectConfigManager.initDatafile)
        // } else {
        //     pollingProjectConfigManager.SyncConfig() // initial poll
        // }
        // return pollingProjectConfigManager
        Self {
            ..PollingProjectConfigManager::default()
        }
    }

    // new_async_polling_project_config_manager returns an instance of the async polling config manager with the customized configuration
    pub fn new_async_polling_project_config_manager(sdk_key: String, polling_manger_options: Vec<String>) -> Self {

        // pollingProjectConfigManager := newConfigManager(sdkKey, pollingMangerOptions...)
        // if len(pollingProjectConfigManager.initDatafile) > 0 {
        //     pollingProjectConfigManager.setInitialDatafile(pollingProjectConfigManager.initDatafile)
        // }
        // return pollingProjectConfigManager
        Self {
            ..PollingProjectConfigManager::default()
        }
    }

    // WithRequester is an optional function, sets a passed requester
    pub fn with_requester(&mut self, requester: Requester) -> Self {
        self.requester = requester;
        self
    }

    /// with_datafile_url_template is an optional function, sets a passed datafile URL template
    pub fn with_datafile_url_template(&mut self, datafile_template: String) -> Self {
        self.datafile_url_template = datafile_template;
        self
    }

    // with_polling_interval is an optional function, sets a passed polling interval
    pub fn with_polling_interval(&mut self, interval: Instant) -> Self {
        self.polling_interval = interval;
        self
    }

    // with_initial_datafile is an optional function, sets a passed datafile
    pub fn with_initial_datafile(&mut self, datafile: Vec<u8>) -> Self {
        self.init_datafile = datafile;
        self
    }

    // with_datafile_access_token is an optional function, sets a passed datafile access token
    pub fn with_datafile_access_token(&mut self, datafile_access_token: String) -> Self {
        p.datafile_access_token = datafile_access_token;
        p
    }

    // sync_config downloads datafile and updates projectConfig
    pub fn sync_config(&self) {
        // var e error
        // var code int
        // var respHeaders http.Header
        // var datafile []byte

        // closeMutex := func(e error) {
        //     cm.err = e
        //     cm.configLock.Unlock()
        // }

        // url := fmt.Sprintf(cm.datafileURLTemplate, cm.sdkKey)
        // if cm.lastModified != "" {
        //     lastModifiedHeader := utils.Header{Name: ModifiedSince, Value: cm.lastModified}
        //     datafile, respHeaders, code, e = cm.requester.Get(url, lastModifiedHeader)
        // } else {
        //     datafile, respHeaders, code, e = cm.requester.Get(url)
        // }

        // if e != nil {
        //     msg := "unable to fetch fresh datafile"
        //     warn!(msg)
        //     cm.configLock.Lock()

        //     if code == http.StatusForbidden {
        //         closeMutex(Err403Forbidden)
        //         return
        //     }

        //     closeMutex(errors.New(fmt.Sprintf("%s, reason (http status code): %s", msg, e.Error())))
        //     return
        // }

        // if code == http.StatusNotModified {
        //     debug!("The datafile was not modified and won't be downloaded again")
        //     return
        // }

        // // Save last-modified date from response header
        // cm.configLock.Lock()
        // lastModified := respHeaders.Get(LastModified)
        // if lastModified != "" {
        //     cm.lastModified = lastModified
        // }

        // projectConfig, err := datafileprojectconfig.NewDatafileProjectConfig(datafile, logging.GetLogger(cm.sdkKey, "NewDatafileProjectConfig"))
        // if err != nil {
        //     warn!("failed to create project config")
        //     closeMutex(errors.New("unable to parse datafile"))
        //     return
        // }

        // var previousRevision string
        // if cm.projectConfig != nil {
        //     previousRevision = cm.projectConfig.GetRevision()
        // }
        // if projectConfig.GetRevision() == previousRevision {
        //     debug!(fmt.Sprintf("No datafile updates. Current revision number: %s", cm.projectConfig.GetRevision()))
        //     closeMutex(nil)
        //     return
        // }
        // err = cm.setConfig(projectConfig)
        // closeMutex(err)
        // if err == nil {
        //     debug!(fmt.Sprintf("New datafile set with revision: %s. Old revision: %s", projectConfig.GetRevision(), previousRevision))
        //     cm.sendConfigUpdateNotification()
        // }
    }

    // Start starts the polling
    pub async fn start(&self) {
        if self.polling_interval <= 0 {
            info!("Polling Config Manager Disabled");
            return
        }
        // debug!("Polling Config Manager Initiated")
        // t := time.NewTicker(cm.pollingInterval)
        // for {
        //     select {
        //     case <-t.C:
        //         cm.SyncConfig()
        //     case <-ctx.Done():
        //         debug!("Polling Config Manager Stopped")
        //         return
        //     }
        // }
    }

    pub fn set_auth_header_if_datafile_access_token_present(&self) {
        if self.datafile_access_token != "" {
            // headers := []utils.Header{{Name: "Content-Type", Value: "application/json"}, {Name: "Accept", Value: "application/json"}}
            // headers = append(headers, utils.Header{Name: "Authorization", Value: "Bearer " + cm.datafileAccessToken})
            // cm.requester = utils.NewHTTPRequester(logging.GetLogger(cm.sdkKey, "HTTPRequester"), utils.Headers(headers...))
        }
    }

    // get_config returns the project config
    pub fn get_config(&mut self) -> Result<dyn ProjectConfig, Error> {
        // cm.configLock.RLock()
        // defer cm.configLock.RUnlock()
        // if cm.projectConfig == nil {
        //     return cm.projectConfig, cm.err
        // }
        // return cm.projectConfig, nil
        Ok(())
    }

    // get_optimizely_config returns the optimizely project config
    pub fn get_optimizely_config(&mut self) -> Option<OptimizelyConfig> {
        // cm.configLock.RLock()
        // defer cm.configLock.RUnlock()
        // if cm.optimizelyConfig != nil {
        //     return cm.optimizelyConfig
        // }
        // optimizelyConfig := NewOptimizelyConfig(cm.projectConfig)
        // cm.optimizelyConfig = optimizelyConfig
        // return cm.optimizelyConfig
        self.optimizely_config
    }

    // on_project_config_update registers a handler for ProjectConfigUpdate notifications
    pub fn on_project_config_update(callback: fn(ProjectConfigUpdateNotification)) -> Result<i64, Error> {
        // handler := func(payload interface{}) {
        //     if projectConfigUpdateNotification, ok := payload.(notification.ProjectConfigUpdateNotification); ok {
        //         callback(projectConfigUpdateNotification)
        //     } else {
        //         warn!(fmt.Sprintf("Unable to convert notification payload %v into ProjectConfigUpdateNotification", payload))
        //     }
        // }
        // id, err := cm.notificationCenter.AddHandler(notification.ProjectConfigUpdate, handler)
        // if err != nil {
        //     warn!("Problem with adding notification handler")
        //     return 0, err
        // }
        // return id, nil
        Ok(1)
    }

    // remove_on_project_config_update removes handler for ProjectConfigUpdate notification with given id
    pub fn remove_on_project_config_update(&mut self, id: i64) -> Result<(), Error> {
        // if err := cm.notificationCenter.RemoveHandler(id, notification.ProjectConfigUpdate); err != nil {
        //     warn!("Problem with removing notification handler")
        //     return err
        // }
        // return nil
        Ok(())
    }

    fn set_config(&mut self, project_config: Box<dyn ProjectConfig>) -> Result<(), Error> {
        // if projectConfig == nil {
        //     return errors.New("unable to set nil config")
        // }
        // cm.projectConfig = projectConfig
        // if cm.optimizelyConfig != nil {
        //     cm.optimizelyConfig = NewOptimizelyConfig(projectConfig)
        // }
        // return nil
        Ok(())
    }

    fn set_initial_datafile(&mut self, datafile: Vec<u8>) {
        // if len(datafile) != 0 {
        //     cm.configLock.Lock()
        //     defer cm.configLock.Unlock()
        //     projectConfig, err := datafileprojectconfig.NewDatafileProjectConfig(datafile, logging.GetLogger(cm.sdkKey, "DatafileProjectConfig"))
        //     if projectConfig != nil {
        //         err = cm.setConfig(projectConfig)
        //     }
        //     cm.err = err
        // }
    }

    fn send_config_update_notification(&self) {
        // if cm.notificationCenter != nil {
        //     projectConfigUpdateNotification := notification.ProjectConfigUpdateNotification{
        //         Type:     notification.ProjectConfigUpdate,
        //         Revision: cm.projectConfig.GetRevision(),
        //     }
        //     if err := cm.notificationCenter.Send(notification.ProjectConfigUpdate, projectConfigUpdateNotification); err != nil {
        //         warn!("Problem with sending notification")
        //     }
        // }
    }
}