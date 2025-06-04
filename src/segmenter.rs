use crate::wal::WalAppender;
use async_stream::stream;
use futures_core::stream::Stream;

pub struct Segmenter {
    pub wal: WalAppender,
}

impl Segmenter {
    pub fn watch(&self) -> impl Stream<Item = ()> + '_ {
        stream! {
            // Placeholder for watching WAL and building segments
        }
    }
}
