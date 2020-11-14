use ring::aead::{
    // quic::AES_128, quic::CHACHA20, 
    AES_128_GCM, AES_256_GCM, CHACHA20_POLY1305,
    SealingKey, UnboundKey, BoundKey, NonceSequence,
    Nonce, Aad,
};
use ring::error::Unspecified;


struct EmptyNonce;

impl NonceSequence for EmptyNonce {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        Ok(Nonce::assume_unique_for_key([
            0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 
            0x05, 0x06, 0x07, 0x08,
        ]))
    }
}

#[bench]
fn aes_128_gcm(b: &mut test::Bencher) {
    let key = [
        0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 
        0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6, 0xb5, 0xf0,
    ];

    let key = UnboundKey::new(&AES_128_GCM, &key).unwrap();
    let mut cipher = SealingKey::new(key, EmptyNonce);

    b.bytes = 16;
    b.iter(|| {
        let mut ciphertext = test::black_box([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 
            0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        ]);
        let _tag = cipher.seal_in_place_separate_tag(Aad::empty(), &mut ciphertext).unwrap();

        ciphertext
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
    
    let key = UnboundKey::new(&AES_256_GCM, &key).unwrap();
    let mut cipher = SealingKey::new(key, EmptyNonce);
    
    b.bytes = 16;
    b.iter(|| {
        let mut ciphertext = test::black_box([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 
            0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        ]);
        let _tag = cipher.seal_in_place_separate_tag(Aad::empty(), &mut ciphertext).unwrap();

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

    let key = UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap();
    let mut cipher = SealingKey::new(key, EmptyNonce);

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
        let _tag = cipher.seal_in_place_separate_tag(Aad::empty(), &mut ciphertext).unwrap();

        ciphertext
    })
}