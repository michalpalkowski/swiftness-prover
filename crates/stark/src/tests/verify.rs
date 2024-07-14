use crate::{
    fixtures::{commitment, domains, witness},
    verify::stark_verify,
};
use cairovm_verifier_air::layout::recursive::{
    RecursiveLayout, NUM_COLUMNS_FIRST, NUM_COLUMNS_SECOND,
};
use cairovm_verifier_fri::fixtures::queries;

#[test]
pub fn test_stark_verify() {
    let queries = queries::get();
    let commitment = commitment::get();
    let witness = witness::get();
    let stark_domains = domains::get();

    stark_verify::<RecursiveLayout>(
        NUM_COLUMNS_FIRST as usize,
        NUM_COLUMNS_SECOND as usize,
        &queries,
        commitment,
        &witness,
        &stark_domains,
    )
    .unwrap()
}