#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PRNG User Entropy Value"]
    pub prng_user_entropy: PRNG_USER_ENTROPY,
    #[doc = "0x04 - PRNG Random Number Output"]
    pub prng_rnd_num: PRNG_RND_NUM,
}
#[doc = "PRNG User Entropy Value"]
pub struct PRNG_USER_ENTROPY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRNG User Entropy Value"]
pub mod prng_user_entropy;
#[doc = "PRNG Random Number Output"]
pub struct PRNG_RND_NUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRNG Random Number Output"]
pub mod prng_rnd_num;
