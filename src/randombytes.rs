use super::ensure_initialized;
use ffi;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Seed([u8; ffi::randombytes_SEEDBYTES as usize]);

pub fn u32() -> u32 {
    ensure_initialized();
    unsafe { ffi::randombytes_random() }
}

pub fn uniform(upper_bound: u32) -> u32 {
    ensure_initialized();
    unsafe { ffi::randombytes_uniform(upper_bound) }
}

pub fn buf_into(out: &mut [u8]) {
    ensure_initialized();
    unsafe {
        ffi::randombytes_buf(out.as_mut_ptr() as *mut _, out.len());
    }
}

pub fn buf(out_len: usize) -> Vec<u8> {
    let mut out = vec![0u8; out_len];
    buf_into(&mut out);
    out
}

pub fn buf_deterministic_into(out: &mut [u8], seed: &Seed) {
    ensure_initialized();
    unsafe {
        ffi::randombytes_buf_deterministic(out.as_mut_ptr() as *mut _, out.len(), seed.0.as_ptr())
    }
}

pub fn buf_deterministic(out_len: usize, seed: &Seed) -> Vec<u8> {
    let mut out = vec![0u8; out_len];
    buf_deterministic_into(&mut out, seed);
    out
}

pub fn ratchet() {
    ensure_initialized();
    unsafe {
        ffi::randombytes_ratchet();
    }
}

pub fn reseed() {
    ensure_initialized();
    unsafe {
        ffi::randombytes_reseed();
    }
}

impl From<[u8; ffi::randombytes_SEEDBYTES as usize]> for Seed {
    fn from(seed: [u8; 32]) -> Seed {
        Seed(seed)
    }
}

impl Into<[u8; ffi::randombytes_SEEDBYTES as usize]> for Seed {
    fn into(self) -> [u8; 32] {
        self.0
    }
}

impl Seed {
    pub fn gen() -> Seed {
        let mut seed_inner = [0u8; ffi::randombytes_SEEDBYTES as usize];
        buf_into(&mut seed_inner);
        Seed(seed_inner)
    }
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_randombytes() {
        init().unwrap();
        assert_ne!(
            randombytes::u32() | randombytes::u32() | randombytes::u32(),
            0
        );

        for _ in 0..100 {
            let max = randombytes::u32();
            assert!(randombytes::uniform(max) < max)
        }

        let len = randombytes::uniform(100) as usize + 1;
        let mut buf = randombytes::buf(len);
        randombytes::buf_into(&mut buf);

        let seed = randombytes::Seed::gen();
        let buf = randombytes::buf_deterministic(len, &seed);
        let mut buf2 = vec![0u8; len];
        randombytes::buf_deterministic_into(&mut buf2, &seed);
        assert_eq!(buf, buf2);

        let seedx: [u8; 32] = seed.into();
        let seedy: randombytes::Seed = seedx.into();
        assert_eq!(seed, seedy);

        randombytes::ratchet();

        randombytes::reseed();
    }
}
