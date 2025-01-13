# translate-telephone
An extremely bare-bones implementation of a "Machine Translate chain" program. Created with my crappy YouTube videos in mind, but it should work for all intents and purposes. Licensed MIT, so feel free (and please do) modify and improve it.

## Details
Written in Rust, automatically making the program blazingly fast™. Uses the crates [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) for API fetching, [`rayon`](https://docs.rs/reqwest/latest/rayon/) for parallelism, and [`serde`](https://serde.rs/) for deserialization. Relies on the official Google Translate API, though a private API key is **not** needed for usage. ([here's how](https://github.com/ssut/py-googletrans/issues/268))