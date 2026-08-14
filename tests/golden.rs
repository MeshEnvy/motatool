//! Frozen on-wire bytes from the pre-optimization encoder.
//!
//! Deployed devices apply these exact detools streams. The in-place encoder may change how it
//! finds matches, but `encode_*` and `build()` must keep producing these containers.

use motatool::endf::ensure_endf;
use motatool::{build, encode, BuildOpts, FwIdent, PatchType};

fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

fn ident() -> FwIdent {
    FwIdent {
        fw_version: 0x0111_0000,
        target_id: 0x04D4_13FD,
        hw_id: "RAK4631".into(),
    }
}

fn synth_bodies() -> (Vec<u8>, Vec<u8>) {
    let base: Vec<u8> = (0..4000u32)
        .map(|i| (i.wrapping_mul(2654435761) >> 13) as u8)
        .collect();
    let mut tgt = base.clone();
    for off in [7usize, 8, 9, 1500, 2600, 3999] {
        tgt[off] ^= 0x5A;
    }
    tgt.extend((0..200u32).map(|i| (i.wrapping_mul(40503) >> 3) as u8));
    (base, tgt)
}

fn prng(seed: u64, n: usize) -> Vec<u8> {
    let mut z = seed;
    (0..n)
        .map(|_| {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut x = z;
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            (x ^ (x >> 31)) as u8
        })
        .collect()
}

fn opts(fw: Vec<u8>, base: Vec<u8>, mem: Option<u32>) -> BuildOpts {
    BuildOpts {
        fw,
        base: Some(base),
        patch_type: PatchType::InPlace,
        inplace_memory: mem,
        segment_size: 0x1000,
        target_id: Some(0x04D4_13FD),
        fw_version: Some(0x0111_0000),
        hw_id: Some("RAK4631".into()),
        sign_seed: None,
        block_size: 1024,
        force: false,
        name_stem: None,
        base_version: None,
    }
}

fn synth_images() -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let (base_body, tgt_body) = synth_bodies();
    let (base_image, _) = ensure_endf(&base_body, &ident());
    let (target_image, _) = ensure_endf(&tgt_body, &ident());
    (base_body, tgt_body, base_image, target_image)
}

#[test]
fn inplace_patch_matches_frozen_encoder() {
    let (_, _, base_image, target_image) = synth_images();
    let patch = encode::encode_in_place(&base_image, &target_image, 0x8000, 0x1000);
    assert_eq!(
        patch,
        fixture("inplace_synth.patch"),
        "in-place patch bytes changed"
    );
}

#[test]
fn sequential_patch_matches_frozen_encoder() {
    let (_, _, base_image, target_image) = synth_images();
    let patch = encode::encode_sequential(&base_image, &target_image);
    assert_eq!(
        patch,
        fixture("sequential_synth.patch"),
        "sequential patch bytes changed"
    );
}

#[test]
fn inplace_mota_matches_frozen_container() {
    let (base_body, tgt_body, base_image, _) = synth_images();
    let built = build(&opts(tgt_body, base_image, Some(0x8000))).unwrap();
    assert_eq!(
        built.bytes,
        fixture("inplace_synth.mota"),
        "in-place .mota container changed"
    );
    let _ = base_body;
}

#[test]
fn inplace_auto_memory_mota_matches_frozen_container() {
    // compute_inplace_memory must reuse the converged patch, not re-encode a different one.
    let (_, tgt_body, base_image, _) = synth_images();
    let built = build(&opts(tgt_body, base_image, None)).unwrap();
    assert_eq!(
        built.bytes,
        fixture("inplace_auto.mota"),
        "auto-memory in-place .mota container changed"
    );
}

#[test]
fn inplace_multiset_patch_matches_frozen_encoder() {
    let n = 20_000usize;
    let base = prng(42 + n as u64, n);
    let mut t = base.clone();
    for k in [0usize, 3, 4, n / 3, n / 2, n - 1] {
        t[k] ^= 0x5A;
    }
    let mem = (((base.len() + t.len()).div_ceil(256)) + 2) as u32 * 256;
    let patch = encode::encode_in_place(&base, &t, mem, 256);
    assert_eq!(
        patch,
        fixture("inplace_20k.patch"),
        "multi-segment in-place patch bytes changed"
    );
}
