use std::collections::HashMap;

use bitcursor::ux::u7;

use crate::node::SharedData;

pub enum MediaType {
    Audio,
    Video,
}

#[derive(Default)]
pub struct StreamInformationStore {
    pub pt_map: HashMap<u7, MediaType>,
}

pub struct AvRtpTransformer {
    stream_information: SharedData<StreamInformationStore>,
}

impl AvRtpTransformer {
    pub fn new(stream_information: SharedData<StreamInformationStore>) -> Self {
        Self { stream_information }
    }
}
