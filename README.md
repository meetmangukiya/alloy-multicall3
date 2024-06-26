# alloy-multicall3

## Usage

```rs
let provider = ...;
let multicall3 = alloy_multicall3::IMulticall3::new(provider, mcall3_addr);

let some_contract = ...;
let some_contract_2 = ...;

// this will call Multicall3::aggregate, can/will probably add macros for other variants
let (call1_result, call2_result, call3_result, call4_result) = multicall!(
    provider,
    multicall3,
    some_contract.some_function(arg1, arg2, arg3),
    some_contract.some_view(arg1, arg2),
    some_contract_2.view(),
    some_contract_2.state_changing_fn(arg1, arg2)
).await?;
```
