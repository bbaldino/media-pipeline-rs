use std::{borrow::Borrow, collections::HashMap, hash::Hash};

use bit_cursor::nsw_types::u7;

use crate::{
    rtp_parser::MediaType,
    util::{LiveStateReader, LiveStateWriter},
};

// TODO: this should map to a 'PayloadType' struct which contains other information
#[derive(Default)]
pub struct PayloadTypes(HashMap<u7, MediaType>);

impl PayloadTypes {
    pub fn insert(&mut self, k: u7, v: MediaType) -> Option<MediaType> {
        self.0.insert(k, v)
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&MediaType>
    where
        u7: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(k)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Default)]
pub struct HeaderExtensionIds(HashMap<String, u8>);

impl HeaderExtensionIds {
    pub fn insert(&mut self, k: String, v: u8) -> Option<u8> {
        self.0.insert(k, v)
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&u8>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(k)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct StreamInformationStore {
    payload_types: LiveStateWriter<PayloadTypes>,
    header_extension_ids: LiveStateWriter<HeaderExtensionIds>,
    // For parties who are interested in only a single mapping
    header_extension_id_writers: HashMap<String, LiveStateWriter<Option<u8>>>,
}

impl StreamInformationStore {
    pub fn new() -> Self {
        let payload_types = LiveStateWriter::new(PayloadTypes::default());
        let header_extension_ids = LiveStateWriter::new(HeaderExtensionIds::default());

        StreamInformationStore {
            payload_types,
            header_extension_ids,
            header_extension_id_writers: HashMap::default(),
        }
    }

    pub fn add_payload_type(&mut self, pt: u7, media_type: MediaType) {
        self.payload_types
            .modify(|pts| _ = pts.insert(pt, media_type));
    }

    pub fn add_payload_types(&mut self, new_pts: HashMap<u7, MediaType>) {
        self.payload_types.modify(|pts| pts.0.extend(new_pts));
    }

    pub fn subscribe_to_pt_changes(&self) -> LiveStateReader<PayloadTypes> {
        self.payload_types.reader()
    }

    pub fn add_header_extension(&mut self, uri: String, id: u8) {
        let uri_insert: String = uri.clone();
        self.header_extension_ids
            .modify(|hes| _ = hes.insert(uri_insert, id));
        if let Some(w) = self.header_extension_id_writers.get(&uri) {
            w.set(Some(id))
        }
    }

    pub fn add_header_extensions(&mut self, new_hes: HashMap<String, u8>) {
        for (uri, id) in &new_hes {
            if let Some(w) = self.header_extension_id_writers.get(uri) {
                w.set(Some(*id))
            }
        }
        self.header_extension_ids
            .modify(|hes| hes.0.extend(new_hes));
    }

    pub fn subscribe_to_header_extension_id_changes(&self) -> LiveStateReader<HeaderExtensionIds> {
        self.header_extension_ids.reader()
    }

    pub fn subscribe_to_header_extension_id_change<T>(
        &mut self,
        uri: T,
    ) -> LiveStateReader<Option<u8>>
    where
        String: Borrow<T>,
        T: Hash + Eq + Into<String>,
    {
        let writer = if let Some(id) = self.header_extension_ids.value().get(&uri) {
            LiveStateWriter::new(Some(*id))
        } else {
            LiveStateWriter::new(None)
        };
        let reader = writer.reader();
        self.header_extension_id_writers.insert(uri.into(), writer);

        reader
    }
}

impl Default for StreamInformationStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_information_store() {
        let mut store = StreamInformationStore::new();

        let he_reader = store.subscribe_to_header_extension_id_changes();
        let pt_reader = store.subscribe_to_pt_changes();
        assert!(he_reader.value().is_empty());
        assert!(pt_reader.value().is_empty());

        store.add_header_extension(String::from("uri"), 10);
        assert_eq!(
            he_reader.value().get(&String::from("uri")),
            Some(10).as_ref()
        );
        assert!(pt_reader.value().is_empty());
        store.add_payload_type(u7::new(100), MediaType::Video);
        assert_eq!(
            pt_reader.value().get(&u7::new(100)),
            Some(MediaType::Video).as_ref()
        );
    }

    #[test]
    fn test_subscribe_to_single_header_ext() {
        let mut store = StreamInformationStore::new();

        let reader = store.subscribe_to_header_extension_id_change(String::from("foo"));

        assert_eq!(*reader.value(), None);
        store.add_header_extension(String::from("foo"), 10);
        assert_eq!(*reader.value(), Some(10));
        let reader2 = store.subscribe_to_header_extension_id_change(String::from("foo"));
        assert_eq!(*reader2.value(), Some(10));
    }
}
