use cipher::NewCipher;

pub fn make_ra_unhappy() {
    let zeroes = [0; 16].into();
    let _cipher = aes::Aes128Ctr::new(&zeroes, &zeroes);
}
