#![feature(macro_metavar_expr)]
#![feature(async_closure)]

use alloy::sol;

sol! {
#[sol(rpc)]
interface IMulticall3 {
struct Call {
    address target;
    bytes callData;
}

struct Call3 {
    address target;
    bool allowFailure;
    bytes callData;
}

struct Call3Value {
    address target;
    bool allowFailure;
    uint256 value;
    bytes callData;
}

struct Result {
    bool success;
    bytes returnData;
}

function aggregate(Call[] calldata calls)
    external
    payable
    view
    returns (uint256 blockNumber, bytes[] memory returnData);

function aggregate3(Call3[] calldata calls) external payable returns (Result[] memory returnData);

function aggregate3Value(Call3Value[] calldata calls)
    external
    payable
    returns (Result[] memory returnData);
}
}

#[macro_export]
macro_rules! multicall {
    ($provider:ident, $multicall3:ident, $( $call: expr ),* ) => {
        async {
            let calls = vec![
                $(
                {
                    let call = $call;
                    let request = call.as_ref();
                    let calldata = call.calldata().clone();
                    let target = alloy::network::TransactionBuilder::to(request).unwrap();
                    $crate::IMulticall3::Call { target, callData: calldata }
                },
                )*
            ];
            let ret = $multicall3.aggregate(calls).call().await?.returnData;
            Ok::<_, Box::<dyn std::error::Error>>((
            $({
                let call = $call;
                let return_data = &ret[${index()}];
                call.decode_output(return_data.clone(), true)
            },
            )*
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use alloy::{primitives::address, providers::ProviderBuilder};
    use tokio::runtime::Runtime;

    #[test]
    fn test_macro() {
        let rt = Runtime::new().unwrap();
        let ret = rt.block_on(async {
            let rpc_url = "https://eth.llamarpc.com".parse().unwrap();
            let provider = ProviderBuilder::new().on_http(rpc_url);
            let multicall3 = super::IMulticall3::new(
                address!("ca11bde05977b3631167028862be2a173976ca11"),
                &provider,
            );
            let (a, b) = multicall!(
                provider_ref,
                multicall3,
                multicall3.aggregate(vec![]),
                multicall3.aggregate3(vec![])
            )
            .await?;
            let a = a?;
            eprintln!(
                "res1 = returnData: {:?}, blockNumber: {:?}, res2 = returnData: {:?}",
                a.returnData,
                a.blockNumber,
                b?.returnData
                    .iter()
                    .map(|ret| format!("ret: {:?}, success: {:?}", ret.returnData, ret.success))
                    .collect::<Vec<String>>()
            );
            Ok::<_, Box<dyn std::error::Error>>(())
        });
        ret.unwrap();
    }
}
