use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;
use spl_governance_addin_api::voter_weight::VoterWeightAction;

mod program_test;

#[tokio::test]
async fn test_update_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let mut voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie)
        .await?;

    let _nft1 = nft_voter_test.token_metadata.with_nft_v2().await;
    let _nft2 = nft_voter_test.token_metadata.with_nft_v2().await;

    nft_voter_test.bench.advance_clock().await;
    let clock = nft_voter_test.bench.get_clock().await;

    // Act
    nft_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            VoterWeightAction::CreateProposal,
        )
        .await?;

    // Assert

    let voter_weight_record = nft_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 10);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CreateProposal)
    );
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}
