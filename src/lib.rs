use aes::Aes128Ctr;
use cipher::NewCipher;

pub fn fail_to_make_ra_unhappy() {
    let zeroes = [0; 16].into();
    let _cipher = Aes128Ctr::new(&zeroes, &zeroes);
}
