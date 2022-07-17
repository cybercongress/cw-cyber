use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use cw1155::{
    ApprovedForAllResponse, BalanceResponse, BatchBalanceResponse, Cw1155BatchReceiveMsg,
    Cw1155ReceiveMsg, IsApprovedForAllResponse,
    TokenInfoResponse, TokensResponse,
};
use cw_neuron_booster::msg::{InstantiateMsg, ExecuteMsg, QueryMsg,
    TokenStateResponse, NeuronVestingsResponse, FundsFromNeuronResponse,
    FundsForNeuronResponse, SwapResponse, FundsResponse
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Cw1155ReceiveMsg), &out_dir);
    export_schema(&schema_for!(Cw1155BatchReceiveMsg), &out_dir);
    export_schema(&schema_for!(BalanceResponse), &out_dir);
    export_schema(&schema_for!(BatchBalanceResponse), &out_dir);
    export_schema(&schema_for!(ApprovedForAllResponse), &out_dir);
    export_schema(&schema_for!(IsApprovedForAllResponse), &out_dir);
    export_schema(&schema_for!(TokenInfoResponse), &out_dir);
    export_schema(&schema_for!(TokensResponse), &out_dir);
    export_schema(&schema_for!(TokenStateResponse), &out_dir);
    export_schema(&schema_for!(NeuronVestingsResponse), &out_dir);
    export_schema(&schema_for!(FundsResponse), &out_dir);
    export_schema(&schema_for!(FundsFromNeuronResponse), &out_dir);
    export_schema(&schema_for!(FundsForNeuronResponse), &out_dir);
    export_schema(&schema_for!(SwapResponse), &out_dir);
}
