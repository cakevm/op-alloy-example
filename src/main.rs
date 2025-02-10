use alloy_provider::{Identity, Provider, ProviderBuilder};
use alloy_rpc_types::{BlockId, BlockTransactionsKind};
use op_alloy_network::Optimism;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_url = "https://base.merkle.io".parse()?;
    let provider = ProviderBuilder::<Identity, Identity, Optimism>::default().on_http(rpc_url);

    let block = provider.get_block(BlockId::latest(), BlockTransactionsKind::Full).await?;
    println!("Block: {:?}", block);

    Ok(())
}
