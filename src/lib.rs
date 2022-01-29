#![feature(
    generators,
    proc_macro_hygiene,
    stmt_expr_attributes,
)]

use cipher::NewCipher;
use aes::cipher::generic_array::GenericArray;
use futures::stream::BoxStream;
use futures_async_stream::stream;

type Aes128Ctr = ctr::Ctr64BE<aes::Aes128>;

pub fn stream_chunks() -> BoxStream<'static, ()> {
    Box::pin(
        #[stream]
        async move {
            let zeroes = GenericArray::from_slice(&[0; 16]);
            let _cipher = Aes128Ctr::new(zeroes, zeroes);
        }
    )
}
