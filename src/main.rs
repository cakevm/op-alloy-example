use alloy_eips::eip1559::BaseFeeParams;
use alloy_primitives::utils::format_units;
use alloy_provider::{Identity, Provider, ProviderBuilder};
use alloy_rpc_types::{BlockId, BlockTransactionsKind};
use op_alloy_consensus::decode_holocene_extra_data;
use op_alloy_network::Optimism;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_url = "https://base.merkle.io".parse()?;
    let provider = ProviderBuilder::<Identity, Identity, Optimism>::default().on_http(rpc_url);

    let Some(block) = provider.get_block(BlockId::latest(), BlockTransactionsKind::Full).await? else {
        return Err(eyre::eyre!("Failed to get block"));
    };

    //
    // Before Holocene
    //
    let Some(next_block_base_fee) = block.header.next_block_base_fee(BaseFeeParams::optimism_canyon()) else {
        return Err(eyre::eyre!("Failed to get base fee"));
    };
    println!("Next block base fee (before Holocene): {} gwei", format_units(next_block_base_fee, "gwei")?);

    //
    // Holocene
    //
    let (elasticity, denominator) = decode_holocene_extra_data(&block.header.extra_data)?;
    let base_fee_params = BaseFeeParams::new(denominator as u128, elasticity as u128);
    let Some(next_block_base_fee) = block.header.next_block_base_fee(base_fee_params) else {
        return Err(eyre::eyre!("Failed to get base fee"));
    };
    println!("Next block base fee (Holocene): {} gwei", format_units(next_block_base_fee, "gwei")?);

    Ok(())
}
