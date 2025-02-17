use aes::{Aes128, Aes256};
use aes::cipher::{NewBlockCipher, BlockEncrypt};
use aes::cipher::generic_array::GenericArray;
use aes_gcm::AesGcm;
use ccm::{Ccm, consts::{U12, U16, U32}};
use aes_gcm::aead::{NewAead, AeadInPlace};
use aes_gcm_siv::AesGcmSiv;
use aes_siv::Aes128SivAead;
use chacha20::ChaCha20;
use chacha20poly1305::ChaCha20Poly1305;
use chacha20::cipher::{NewCipher, StreamCipher};

type Aes128Gcm    = AesGcm<Aes128, U12>;
type Aes256Gcm    = AesGcm<Aes256, U12>;
type Aes128Ccm    = Ccm<Aes128, U16, U12>; // AEAD_AES_128_CCM  NONCE-LEN=12, TAG-LEN=16, Q=3
type Aes128GcmSiv = AesGcmSiv<Aes128>;

type ChaCha20Key           = GenericArray<u8, U32>;
type ChaCha20Nonce         = GenericArray<u8, U12>;
type ChaCha20Poly1305Key   = GenericArray<u8, U32>;
type ChaCha20Poly1305Nonce = GenericArray<u8, U12>;

#[bench]
fn aes_128(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
    ];

    let key    = GenericArray::from(key);
    let cipher = Aes128::new(&key);

    b.bytes = 16;
    b.iter(|| {
        let mut ciphertext = test::black_box(GenericArray::from([
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
        ]));
        cipher.encrypt_block(&mut ciphertext);
        ciphertext
    })
}

#[bench]
fn aes_256(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
    ];

    let key    = GenericArray::from(key);
    let cipher = Aes256::new(&key);

    b.bytes = 16;
    b.iter(|| {
        let mut ciphertext = test::black_box(GenericArray::from([
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
        ]));
        cipher.encrypt_block(&mut ciphertext);
        ciphertext
    })
}

#[bench]
fn aes_128_gcm(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
    ];
    let nonce = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 
        0x05, 0x06, 0x07, 0x08,
    ];
    let aad = [0u8; 0];

    let key   = GenericArray::from(key);
    let nonce = GenericArray::from(nonce);

    let cipher = Aes128Gcm::new(&key);

    b.bytes = 16;
    b.iter(|| {
        let mut ciphertext = test::black_box([
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
        ]);
        let tag = cipher.encrypt_in_place_detached(&nonce, &aad, &mut ciphertext).unwrap();
        tag
    })
}

#[bench]
fn aes_256_gcm(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
    ];
    let nonce = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 
        0x05, 0x06, 0x07, 0x08,
    ];
    let aad = [0u8; 0];

    let key    = GenericArray::from(key);
    let nonce  = GenericArray::from(nonce);
    let cipher = Aes256Gcm::new(&key);
    
    b.bytes = 16;
    b.iter(|| {
        let mut ciphertext = test::black_box([
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
        ]);
        let tag = cipher.encrypt_in_place_detached(&nonce, &aad, &mut ciphertext).unwrap();
        tag
    })
}

#[bench]
fn aes_128_ccm(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
    ];
    let nonce = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 
        0x05, 0x06, 0x07, 0x08,
    ];
    let aad = [0u8; 0];

    let key   = GenericArray::from(key);
    let nonce = GenericArray::from(nonce);

    let cipher = Aes128Ccm::new(&key);

    b.bytes = 16;
    b.iter(|| {
        let mut ciphertext = test::black_box([
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
        ]);
        let tag = cipher.encrypt_in_place_detached(&nonce, &aad, &mut ciphertext).unwrap();
        tag
    })
}

#[bench]
fn aes_128_gcm_siv(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
    ];
    let nonce = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 
        0x05, 0x06, 0x07, 0x08,
    ];
    let aad = [0u8; 0];

    let key   = GenericArray::from(key);
    let nonce = GenericArray::from(nonce);

    let cipher = Aes128GcmSiv::new(&key);

    b.bytes = 16;
    b.iter(|| {
        let mut ciphertext = test::black_box([
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
        ]);
        let tag = cipher.encrypt_in_place_detached(&nonce, &aad, &mut ciphertext).unwrap();
        tag
    })
}

#[bench]
fn aes_siv_cmac_256(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
    ];
    // NOTE: AEAD_AES_SIV_CMAC_256 标准并不一定需要 NONCE.
    //       RustCrypto 组织把 Nonce 作为一个必选项，
    //       并且设置了一个固定长度，这可能会导致和其它实现的冲突。
    let nonce = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 
        0x05, 0x06, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 
    ];
    let aad = [0u8; 0];

    let key   = GenericArray::from(key);
    let nonce = GenericArray::from(nonce);

    // AEAD_AES_SIV_CMAC_256
    let cipher = Aes128SivAead::new(&key);

    b.bytes = 16;
    b.iter(|| {
        let mut ciphertext = test::black_box([
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0, 
        ]);
        let tag = cipher.encrypt_in_place_detached(&nonce, &aad, &mut ciphertext).unwrap();
        tag
    })
}


#[bench]
fn chacha20(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
        0x47, 0x39, 0x17, 0xc1, 0x40, 0x2b, 0x80, 0x09, 
        0x9d, 0xca, 0x5c, 0xbc, 0x20, 0x70, 0x75, 0xc0,
    ];
    let nonce = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 
        0x05, 0x06, 0x07, 0x08,
    ];

    let key   = ChaCha20Key::from_slice(&key);
    let nonce = ChaCha20Nonce::from_slice(&nonce);
    
    b.bytes = 64;
    b.iter(|| {
        let mut cipher = ChaCha20::new(key, nonce);

        let mut ciphertext = test::black_box([
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
            0x47, 0x39, 0x17, 0xc1, 0x40, 0x2b, 0x80, 0x09, 
            0x9d, 0xca, 0x5c, 0xbc, 0x20, 0x70, 0x75, 0xc0,
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
            0x47, 0x39, 0x17, 0xc1, 0x40, 0x2b, 0x80, 0x09,
            0x9d, 0xca, 0x5c, 0xbc, 0x20, 0x70, 0x75, 0xc0,
        ]);
        cipher.apply_keystream(&mut ciphertext);
        ciphertext
    })
}


#[bench]
fn chacha20_poly1305(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
        0x47, 0x39, 0x17, 0xc1, 0x40, 0x2b, 0x80, 0x09, 
        0x9d, 0xca, 0x5c, 0xbc, 0x20, 0x70, 0x75, 0xc0,
    ];
    let nonce = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 
        0x05, 0x06, 0x07, 0x08,
    ];
    let aad = [0u8; 0];

    let key   = ChaCha20Poly1305Key::from_slice(&key);
    let nonce = ChaCha20Poly1305Nonce::from_slice(&nonce);

    let cipher = ChaCha20Poly1305::new(key);

    b.bytes = 64;
    b.iter(|| {
        let mut ciphertext = test::black_box([
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
            0x47, 0x39, 0x17, 0xc1, 0x40, 0x2b, 0x80, 0x09, 
            0x9d, 0xca, 0x5c, 0xbc, 0x20, 0x70, 0x75, 0xc0,
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
            0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
            0x47, 0x39, 0x17, 0xc1, 0x40, 0x2b, 0x80, 0x09, 
            0x9d, 0xca, 0x5c, 0xbc, 0x20, 0x70, 0x75, 0xc0,
        ]);
        let tag = cipher.encrypt_in_place_detached(&nonce, &aad, &mut ciphertext).unwrap();
        tag
    })
}
