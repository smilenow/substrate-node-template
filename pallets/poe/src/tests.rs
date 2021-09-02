use super::*;
use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn postive_case_create_claim() {
    new_test_ext().execute_with(|| {
        let claim = vec![1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1u64), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            (1u64, frame_system::Pallet::<Test>::block_number())
        );
    })
}

#[test]
fn negative_case_create_claim_failed_when_claim_already_exists() {
    new_test_ext().execute_with(|| {
        let claim = vec![1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1u64), claim.clone()));
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1u64), claim.clone()),
            Error::<Test>::ProofAlreadyClaimedError
        );
    })
}

#[test]
fn postive_case_revoke_claim() {
    new_test_ext().execute_with(|| {
        let claim = vec![1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1u64), claim.clone()));
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1u64), claim.clone()));
        assert_eq!(PoeModule::proofs(&claim), (0u64, 0u64));
    })
}

#[test]
fn negative_case_revoke_claim_failed_when_claim_does_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1u64), claim.clone()),
            Error::<Test>::ProofNotFoundError
        );
    })
}

#[test]
fn negative_case_revoke_claim_failed_when_he_is_not_the_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1u64), claim.clone()));
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2u64), claim.clone()),
            Error::<Test>::ProofOwnershipError
        );
    })
}

#[test]
fn postive_case_transfer_claim() {
    new_test_ext().execute_with(|| {
        let claim = vec![1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1u64), claim.clone()));
        assert_ok!(PoeModule::transfer_claim(
            Origin::signed(1),
            claim.clone(),
            2u64
        ));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            (2u64, frame_system::Module::<Test>::block_number())
        );
    })
}

#[test]
fn negative_case_transfer_claim_failed_when_claim_does_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![1];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1u64), claim.clone(), 2u64),
            Error::<Test>::ProofNotFoundError
        );
    })
}

#[test]
fn negative_case_transfer_claim_failed_with_a_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1u64), claim.clone()));
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2u64), claim.clone(), 3u64),
            Error::<Test>::ProofOwnershipError
        );
    })
}

#[test]
fn negative_case_create_claim_failed_when_claim_exceed_size() {
    new_test_ext().execute_with(|| {
        let claim = vec![1; 100];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1u64), claim.clone()),
            Error::<Test>::ProofSizeExceededError,
        );
    });
}