use cipher::NewCipher;

type Aes128Ctr = ctr::Ctr64BE<aes::Aes128>;

pub fn make_ra_unhappy() {
    let zeroes = [0; 16].into();
    let _cipher = Aes128Ctr::new(&zeroes, &zeroes);
}
