pub struct Client {
    pub sdk_key: String
}

impl Client {
    fn new(sdk_key: String) -> Self {
        Self {
            sdk_key: sdk_key
        }
    }
}