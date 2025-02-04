use async_trait::async_trait;
use kaspa_notify::events::EVENT_TYPE_ARRAY;
use kaspa_notify::listener::ListenerId;
use kaspa_notify::notifier::Notifier;
use kaspa_notify::scope::Scope;
use kaspa_rpc_core::{api::rpc::RpcApi, *};
use kaspa_rpc_core::{notify::connection::ChannelConnection, RpcResult};
use std::sync::Arc;

pub(super) type RpcCoreNotifier = Notifier<Notification, ChannelConnection>;

pub(super) struct RpcCoreMock {
    core_notifier: Arc<RpcCoreNotifier>,
}

impl RpcCoreMock {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn core_notifier(&self) -> Arc<RpcCoreNotifier> {
        self.core_notifier.clone()
    }

    pub(super) fn start(&self) {
        self.core_notifier.clone().start();
    }
    pub(super) async fn join(&self) {
        self.core_notifier.join().await.unwrap();
    }
}

impl Default for RpcCoreMock {
    fn default() -> Self {
        let core_notifier: Arc<RpcCoreNotifier> = Arc::new(Notifier::new("rpc-core", EVENT_TYPE_ARRAY[..].into(), vec![], vec![], 1));
        Self { core_notifier }
    }
}

#[async_trait]
impl RpcApi for RpcCoreMock {
    // This fn needs to succeed while the client connects
    async fn get_info_call(&self, _request: GetInfoRequest) -> RpcResult<GetInfoResponse> {
        Ok(GetInfoResponse {
            p2p_id: "p2p-mock".to_string(),
            mempool_size: 1234,
            server_version: "mock".to_string(),
            is_utxo_indexed: false,
            is_synced: false,
            has_notify_command: false,
            has_message_id: false,
        })
    }

    async fn ping_call(&self, _request: PingRequest) -> RpcResult<PingResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_process_metrics_call(&self, _request: GetProcessMetricsRequest) -> RpcResult<GetProcessMetricsResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_current_network_call(&self, _request: GetCurrentNetworkRequest) -> RpcResult<GetCurrentNetworkResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn submit_block_call(&self, _request: SubmitBlockRequest) -> RpcResult<SubmitBlockResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_block_template_call(&self, _request: GetBlockTemplateRequest) -> RpcResult<GetBlockTemplateResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_peer_addresses_call(&self, _request: GetPeerAddressesRequest) -> RpcResult<GetPeerAddressesResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_selected_tip_hash_call(&self, _request: GetSelectedTipHashRequest) -> RpcResult<GetSelectedTipHashResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_mempool_entry_call(&self, _request: GetMempoolEntryRequest) -> RpcResult<GetMempoolEntryResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_mempool_entries_call(&self, _request: GetMempoolEntriesRequest) -> RpcResult<GetMempoolEntriesResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_connected_peer_info_call(&self, _request: GetConnectedPeerInfoRequest) -> RpcResult<GetConnectedPeerInfoResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn add_peer_call(&self, _request: AddPeerRequest) -> RpcResult<AddPeerResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn submit_transaction_call(&self, _request: SubmitTransactionRequest) -> RpcResult<SubmitTransactionResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_block_call(&self, _request: GetBlockRequest) -> RpcResult<GetBlockResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_subnetwork_call(&self, _request: GetSubnetworkRequest) -> RpcResult<GetSubnetworkResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_virtual_chain_from_block_call(
        &self,
        _request: GetVirtualChainFromBlockRequest,
    ) -> RpcResult<GetVirtualChainFromBlockResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_blocks_call(&self, _request: GetBlocksRequest) -> RpcResult<GetBlocksResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_block_count_call(&self, _request: GetBlockCountRequest) -> RpcResult<GetBlockCountResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_block_dag_info_call(&self, _request: GetBlockDagInfoRequest) -> RpcResult<GetBlockDagInfoResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn resolve_finality_conflict_call(
        &self,
        _request: ResolveFinalityConflictRequest,
    ) -> RpcResult<ResolveFinalityConflictResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn shutdown_call(&self, _request: ShutdownRequest) -> RpcResult<ShutdownResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_headers_call(&self, _request: GetHeadersRequest) -> RpcResult<GetHeadersResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_balance_by_address_call(&self, _request: GetBalanceByAddressRequest) -> RpcResult<GetBalanceByAddressResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_balances_by_addresses_call(
        &self,
        _request: GetBalancesByAddressesRequest,
    ) -> RpcResult<GetBalancesByAddressesResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_utxos_by_addresses_call(&self, _request: GetUtxosByAddressesRequest) -> RpcResult<GetUtxosByAddressesResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_sink_blue_score_call(&self, _request: GetSinkBlueScoreRequest) -> RpcResult<GetSinkBlueScoreResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn ban_call(&self, _request: BanRequest) -> RpcResult<BanResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn unban_call(&self, _request: UnbanRequest) -> RpcResult<UnbanResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn estimate_network_hashes_per_second_call(
        &self,
        _request: EstimateNetworkHashesPerSecondRequest,
    ) -> RpcResult<EstimateNetworkHashesPerSecondResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_mempool_entries_by_addresses_call(
        &self,
        _request: GetMempoolEntriesByAddressesRequest,
    ) -> RpcResult<GetMempoolEntriesByAddressesResponse> {
        Err(RpcError::NotImplemented)
    }

    async fn get_coin_supply_call(&self, _request: GetCoinSupplyRequest) -> RpcResult<GetCoinSupplyResponse> {
        Err(RpcError::NotImplemented)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Notification API

    fn register_new_listener(&self, connection: ChannelConnection) -> ListenerId {
        self.core_notifier.register_new_listener(connection)
    }

    async fn unregister_listener(&self, id: ListenerId) -> RpcResult<()> {
        self.core_notifier.unregister_listener(id)?;
        Ok(())
    }

    async fn start_notify(&self, _id: ListenerId, _scope: Scope) -> RpcResult<()> {
        Err(RpcError::NotImplemented)
    }

    async fn stop_notify(&self, _id: ListenerId, _scope: Scope) -> RpcResult<()> {
        Err(RpcError::NotImplemented)
    }
}
