# Changelog

## 24.0.0 - 2025-10-09

**Derivatives Trading Coin Futures**

### Changed (1)

#### REST API

- Modified response for `query_order()` (`GET /dapi/v1/order`):
  - property `positionSide` added

**Derivatives Trading Options**

### Changed (4)

#### REST API

- Deleted parameter `price`
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)
- Deleted parameter `quantity`
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)
- Deleted parameter `side`
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)
- Deleted parameter `symbol`
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)

**Spot**

### Changed (4)

#### REST API

- Modified response for `exchange_info()` (`GET /api/v3/exchangeInfo`):
  - modified `exchange_filters` and `symbols`.`filters`

- Modified response for `my_filters()` (`GET /api/v3/myFilters`):
  - modified `asset_filters`, `exchange_filters` and `symbol_filters`

#### WebSocket API

- Modified response for `exchange_info()` (`exchangeInfo` method):
  - modified `result`.`exchange_filters` and `result`.`symbols`.`filters`

- Modified response for `my_filters()` (`myFilters` method):
  - modified `result`.`asset_filters`, `result`.`exchange_filters` and `result`.`symbol_filters`

## 23.0.0 - 2025-10-06

**Derivatives Trading Options**

### Changed (1)

#### REST API

- Deleted parameter `limit`
  - affected methods:
    - `query_current_open_option_orders()` (`GET /eapi/v1/openOrders`)

**Sub Account**

### Changed (1)

- Modified parameter `orderArgs`:
  - item property `positionSide` added
  - item property `quantity` added
  - item property `symbol` added
  - affected methods:
    - `move_position_for_sub_account()` (`POST /sapi/v1/sub-account/futures/move-position`)

## 22.0.0 - 2025-09-29

**Derivatives Trading Portfolio Margin Pro**

### Changed (2)

#### REST API

- Modified response for `mint_bfusd_for_portfolio_margin()` (`POST /sapi/v1/portfolio/mint`):
  - property `mintRate` added
  - property `rate` deleted

- Modified response for `redeem_bfusd_for_portfolio_margin()` (`POST /sapi/v1/portfolio/redeem`):
  - property `redeemRate` added
  - property `rate` deleted

## 21.0.0 - 2025-09-24

**Spot**

### Changed (2)

#### WebSocket API

- Modified parameter `belowTimeInForce`:
  - enum removed: `belowType`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT_LIMIT`
  - enum added: `GTC`, `IOC`, `FOK`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)

## 20.0.0 - 2025-09-19

**Convert**

### Changed (1)

- Update `query_limit_open_orders()` HTTP method (`GET` from `POST`)

**Spot**

### Changed (2)

#### REST API

- Modified parameter `recvWindow`:
  - type `integer` → `number`
  - format `int64` → `float`
  - affected methods:
    - `get_account()` (`GET /api/v3/account`)
    - `all_order_list()` (`GET /api/v3/allOrderList`)
    - `all_orders()` (`GET /api/v3/allOrders`)
    - `my_allocations()` (`GET /api/v3/myAllocations`)
    - `my_prevented_matches()` (`GET /api/v3/myPreventedMatches`)
    - `my_trades()` (`GET /api/v3/myTrades`)
    - `open_order_list()` (`GET /api/v3/openOrderList`)
    - `delete_open_orders()` (`DELETE /api/v3/openOrders`)
    - `get_open_orders()` (`GET /api/v3/openOrders`)
    - `delete_order()` (`DELETE /api/v3/order`)
    - `get_order()` (`GET /api/v3/order`)
    - `new_order()` (`POST /api/v3/order`)
    - `order_amend_keep_priority()` (`PUT /api/v3/order/amend/keepPriority`)
    - `order_amendments()` (`GET /api/v3/order/amendments`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_oco()` (`POST /api/v3/order/oco`)
    - `order_test()` (`POST /api/v3/order/test`)
    - `delete_order_list()` (`DELETE /api/v3/orderList`)
    - `get_order_list()` (`GET /api/v3/orderList`)
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
    - `rate_limit_order()` (`GET /api/v3/rateLimit/order`)
    - `sor_order()` (`POST /api/v3/sor/order`)
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)

#### WebSocket API

- Modified parameter `recvWindow`:
  - type `integer` → `number`
  - format `int64` → `float`
  - affected methods:
    - `account_rate_limits_orders()` (`account.rateLimits.orders` method)
    - `account_status()` (`account.status` method)
    - `all_order_lists()` (`allOrderLists` method)
    - `all_orders()` (`allOrders` method)
    - `my_allocations()` (`myAllocations` method)
    - `my_prevented_matches()` (`myPreventedMatches` method)
    - `my_trades()` (`myTrades` method)
    - `open_order_lists_status()` (`openOrderLists.status` method)
    - `open_orders_cancel_all()` (`openOrders.cancelAll` method)
    - `open_orders_status()` (`openOrders.status` method)
    - `order_amend_keep_priority()` (`order.amend.keepPriority` method)
    - `order_amendments()` (`order.amendments` method)
    - `order_cancel()` (`order.cancel` method)
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_status()` (`order.status` method)
    - `order_test()` (`order.test` method)
    - `order_list_cancel()` (`orderList.cancel` method)
    - `order_list_place()` (`orderList.place` method)
    - `order_list_place_oco()` (`orderList.place.oco` method)
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
    - `order_list_status()` (`orderList.status` method)
    - `session_logon()` (`session.logon` method)
    - `sor_order_place()` (`sor.order.place` method)
    - `sor_order_test()` (`sor.order.test` method)

**Sub Account**

### Changed (1)

- Modified parameter `email`:
  - required: `true` → `false`
  - affected methods:
    - `query_sub_account_transaction_statistics()` (`GET /sapi/v1/sub-account/transaction-statistics`)

## 19.0.0 - 2025-09-17

**Derivatives Trading Portfolio Margin**

### Changed (2)

#### REST API

- Modified response for `margin_max_borrow()` (`GET /papi/v1/margin/maxBorrowable`):
  - `amount`: type `number` → `string`
  - `borrowLimit`: type `integer` → `string`

- Modified response for `new_margin_order()` (`POST /papi/v1/margin/order`):
  - `marginBuyBorrowAmount`: type `integer` → `string`

## 18.0.0 - 2025-09-15

**Pay**

### Changed (1)

- Modified response for `get_pay_trade_history()` (`GET /sapi/v1/pay/transactions`):
  - `data`.`payerInfo`: property `accountId` deleted

**Wallet**

### Changed (1)

- Modified response for `deposit_history()` (`GET /sapi/v1/capital/deposit/hisrec`):
  - item property `travelRuleStatus` added

## 17.0.0 - 2025-09-09

**Staking**

### Added (3)

- `get_soft_staking_product_list()` (`GET /sapi/v1/soft-staking/list`)
- `get_soft_staking_rewards_history()` (`GET /sapi/v1/soft-staking/history/rewardsRecord`)
- `set_soft_staking()` (`GET /sapi/v1/soft-staking/set`)

**Wallet**

### Changed (1)

- Modified response for `all_coins_information()` (`GET /sapi/v1/capital/config/getall`):
  - `networkList`: item property `withdrawTag` added

## 16.0.0 - 2025-09-05

**Simple Earn**

### Added (8)

- `get_rwusd_account()` (`GET /sapi/v1/rwusd/account`)
- `get_rwusd_quota_details()` (`GET /sapi/v1/rwusd/quota`)
- `get_rwusd_rate_history()` (`GET /sapi/v1/rwusd/history/rateHistory`)
- `get_rwusd_redemption_history()` (`GET /sapi/v1/rwusd/history/redemptionHistory`)
- `get_rwusd_rewards_history()` (`GET /sapi/v1/rwusd/history/rewardsHistory`)
- `get_rwusd_subscription_history()` (`GET /sapi/v1/rwusd/history/subscriptionHistory`)
- `redeem_rwusd()` (`POST /sapi/v1/rwusd/redeem`)
- `subscribe_rwusd()` (`POST /sapi/v1/rwusd/subscribe`)

## 15.0.0 - 2025-09-05

**Derivatives Trading Usds Futures**

### Changed (1)

#### REST API

- Modified response for `notional_and_leverage_brackets()` (`GET /fapi/v1/leverageBracket`):

## 14.0.0 - 2025-08-29

**Simple Earn**

### Changed (1)

- Modified response for `get_simple_earn_locked_product_list()` (`GET /sapi/v1/simple-earn/locked/list`):
  - `rows`.`detail`.`boostEndTime`: type `string` → `integer`

**Wallet**

### Added (1)

- `deposit_history_v2()` (`GET /sapi/v2/localentity/deposit/history`)

## 13.0.0 - 2025-08-26

**Crypto Loan**

### Changed (2)

- Added parameter `collateralAmount`
  - affected methods:
    - `flexible_loan_borrow()` (`POST /sapi/v2/loan/flexible/borrow`)
- Added parameter `loanAmount`
  - affected methods:
    - `flexible_loan_borrow()` (`POST /sapi/v2/loan/flexible/borrow`)

**Derivatives Trading Usds Futures**

### Changed (1)

#### REST API

- Modified response for `account_information_v3()` method (`GET /fapi/v3/account`):
  - `assets`: item property `marginAvailable` deleted

**Vip Loan**

### Changed (1)

- Added parameter `loanTerm`
  - affected methods:
    - `vip_loan_borrow()` (`POST /sapi/v1/loan/vip/borrow`)

## 12.0.0 - 2025-08-21

**Derivatives Trading Coin Futures**

### Changed (1)

#### REST API

- Modified response for `exchange_information()` method (`GET /dapi/v1/exchangeInfo`):
  - `symbols`.`filters`.`multiplierDecimal`: type `integer` → `string`

**Margin Trading**

### Added (1)

#### REST API

- `get_limit_price_pairs()` (`GET /sapi/v1/margin/limit-price-pairs`)

**Simple Earn**

### Changed (2)

- Modified response for `get_simple_earn_flexible_product_list()` method (`GET /sapi/v1/simple-earn/flexible/list`):
  - `rows`.`subscriptionStartTime`: type `string` → `integer`

- Modified response for `get_simple_earn_locked_product_list()` method (`GET /sapi/v1/simple-earn/locked/list`):

**Staking**

### Changed (1)

- Modified response for `get_on_chain_yields_locked_product_list()` method (`GET /sapi/v1/onchain-yields/locked/list`):
  - `rows`.`detail`.`subscriptionStartTime`: type `string` → `integer`

**Spot**

### Added (2)

#### WebSocket API

- `session_subscriptions()` (`session.subscriptions` method)
- `user_data_stream_subscribe_signature()` (`userDataStream.subscribe.signature` method)

### Changed (83)

#### REST API

- Added parameter `abovePegOffsetType`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `abovePegOffsetValue`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `abovePegPriceType`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `belowPegOffsetType`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `belowPegOffsetValue`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `belowPegPriceType`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `icebergQty`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `newClientOrderId`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `newOrderRespType`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `pegOffsetType`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
- Added parameter `pegOffsetValue`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
- Added parameter `pegPriceType`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
- Added parameter `pendingAbovePegOffsetType`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingAbovePegOffsetValue`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingAbovePegPriceType`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingBelowPegOffsetType`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingBelowPegOffsetValue`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingBelowPegPriceType`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingPegOffsetType`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
- Added parameter `pendingPegOffsetValue`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
- Added parameter `pendingPegPriceType`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
- Added parameter `price`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `quantity`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `recvWindow`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `selfTradePreventionMode`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `side`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `strategyId`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `strategyType`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `symbol`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `timeInForce`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `type`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `workingPegOffsetType`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `workingPegOffsetValue`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `workingPegPriceType`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Modified parameter `computeCommissionRates`:
  - affected methods:
    - `order_test()` (`POST /api/v3/order/test`)
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)

- Modified response for `account_commission()` method (`GET /api/v3/account/commission`):
  - property `specialCommission` added

- Modified response for `exchange_info()` method (`GET /api/v3/exchangeInfo`):
  - `exchangeFilters`: item property `maxNumOrderAmends` added
  - `exchangeFilters`: item property `maxNumOrderLists` added
  - `symbols`: item property `pegInstructionsAllowed` added
  - `symbols`.`filters`: item property `maxNumOrderAmends` added
  - `symbols`.`filters`: item property `maxNumOrderLists` added

- Modified response for `order_test()` method (`POST /api/v3/order/test`):
  - property `specialCommissionForOrder` added

#### WebSocket API

- Added parameter `abovePegOffsetType`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `abovePegOffsetValue`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `abovePegPriceType`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `belowPegOffsetType`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `belowPegOffsetValue`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `belowPegPriceType`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `icebergQty`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `newClientOrderId`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `newOrderRespType`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `pegOffsetType`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
- Added parameter `pegOffsetValue`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
- Added parameter `pegPriceType`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
- Added parameter `pendingAbovePegOffsetType`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingAbovePegOffsetValue`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingAbovePegPriceType`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingBelowPegOffsetType`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingBelowPegOffsetValue`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingBelowPegPriceType`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingPegOffsetType`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
- Added parameter `pendingPegOffsetValue`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
- Added parameter `pendingPegPriceType`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
- Added parameter `price`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `quantity`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `quoteOrderQty`
  - affected methods:
    - `order_test()` (`order.test` method)
- Added parameter `recvWindow`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `selfTradePreventionMode`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `side`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `stopPrice`
  - affected methods:
    - `order_test()` (`order.test` method)
- Added parameter `strategyId`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `strategyType`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `subscriptionId`
  - affected methods:
    - `user_data_stream_unsubscribe()` (`userDataStream.unsubscribe` method)
- Added parameter `symbol`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `timeInForce`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `trailingDelta`
  - affected methods:
    - `order_test()` (`order.test` method)
- Added parameter `type`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `workingPegOffsetType`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `workingPegOffsetValue`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `workingPegPriceType`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Modified parameter `cancelOrderId`:
  - format `int32` → `int64`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
- Modified parameter `computeCommissionRates`:
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Modified parameter `orderId`:
  - format `int32` → `int64`
  - affected methods:
    - `all_orders()` (`allOrders` method)
    - `my_trades()` (`myTrades` method)
    - `order_cancel()` (`order.cancel` method)
    - `order_status()` (`order.status` method)

- Modified response for `account_commission()` method (`account.commission`):
  - `result`: property `specialCommission` added

- Modified response for `exchange_info()` method (`exchangeInfo`):
  - `result`.`exchangeFilters`: item property `maxNumOrderLists` added
  - `result`.`exchangeFilters`: item property `maxNumOrderAmends` added
  - `result`.`symbols`: item property `pegInstructionsAllowed` added
  - `result`.`symbols`.`filters`: item property `maxNumOrderLists` added
  - `result`.`symbols`.`filters`: item property `maxNumOrderAmends` added

- Modified response for `order_test()` method (`order.test`):
  - `result`: property `specialCommissionForOrder` added

- Modified response for `user_data_stream_subscribe()` method (`userDataStream.subscribe`):
  - `result`: property `subscriptionId` added

## 11.0.0 - 2025-07-29

### Changed (1)

- Enhanced WebSocket reconnection logic with intelligent error classification.

## 10.0.0 - 2025-07-23

**Spot**

### Changed (4)

#### REST API

- Added missing parameters to `order_test()` (`POST /api/v3/order/test`)

#### WebSocket API

- Added missing parameters to `order_test()`

- Modified parameter `cancelOrderId`:
  - format `int32` → `int64`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
- Modified parameter `orderId`:
  - format `int32` → `int64`
  - affected methods:
    - `all_orders()` (`allOrders` method)
    - `my_trades()` (`myTrades` method)
    - `order_cancel()` (`order.cancel` method)
    - `order_status()` (`order.status` method)

## 9.0.0 - 2025-07-22

### Added (1)

**Wallet**

- `check_questionnaire_requirements()` (`GET /sapi/v1/localentity/questionnaire-requirements`)

### Changed (4)

**Derivatives Trading Options**

#### REST API

- Modified response for `exchange_information()` method (`GET /eapi/v1/exchangeInfo`):
  - `optionSymbols`: item property `liquidationFeeRate` added

- Modified response for `option_margin_account_information()` method (`GET /eapi/v1/marginAccount`):
  - `asset`: item property `adjustedEquity` added
  - `asset`: item property `lpProfit` deleted

**Wallet**

- Added parameter `recvWindow`
  - affected methods:
    - `fetch_address_verification_list()` (`GET /sapi/v1/addressVerify/list`)
    - `vasp_list()` (`GET /sapi/v1/localentity/vasp`)

**Spot**

#### REST API

- Added missing parameters for `order_test()` (`POST /api/v3/order/test`)

## 8.0.0 - 2025-07-14

### Added (1)

- Support session management for WebSocket API (where supported), with auto session re-logon (`auto_session_relogon` option on `ConfigurationWebSocketApi`).

### Changed (2)

- Fixed bug on URL query params generation function.

**Wallet**

- Modified response for `all_coins_information()` method (`GET /sapi/v1/capital/config/getall`):

## 7.0.0 - 2025-07-08

### Added (14)

- Support User, Risk and Trade Data Stream Events for Derivatives, Margin Trading and Spot.
- Support custom headers on REST API requests (`custom_headers` option on `ConfigurationRestApi`).

**Staking**

- `get_on_chain_yields_locked_personal_left_quota()` (`GET /sapi/v1/onchain-yields/locked/personalLeftQuota`)
- `get_on_chain_yields_locked_product_list()` (`GET /sapi/v1/onchain-yields/locked/list`)
- `get_on_chain_yields_locked_product_position()` (`GET /sapi/v1/onchain-yields/locked/position`)
- `get_on_chain_yields_locked_redemption_record()` (`GET /sapi/v1/onchain-yields/locked/history/redemptionRecord`)
- `get_on_chain_yields_locked_rewards_history()` (`GET /sapi/v1/onchain-yields/locked/history/rewardsRecord`)
- `get_on_chain_yields_locked_subscription_preview()` (`GET /sapi/v1/onchain-yields/locked/subscriptionPreview`)
- `get_on_chain_yields_locked_subscription_record()` (`GET /sapi/v1/onchain-yields/locked/history/subscriptionRecord`)
- `on_chain_yields_account()` (`GET /sapi/v1/onchain-yields/account`)
- `redeem_on_chain_yields_locked_product()` (`POST /sapi/v1/onchain-yields/locked/redeem`)
- `set_on_chain_yields_locked_auto_subscribe()` (`POST /sapi/v1/onchain-yields/locked/setAutoSubscribe`)
- `set_on_chain_yields_locked_product_redeem_option()` (`POST /sapi/v1/onchain-yields/locked/setRedeemOption`)
- `subscribe_on_chain_yields_locked_product()` (`POST /sapi/v1/onchain-yields/locked/subscribe`)

### Changed (3)

- Fixed bug with Ed25519 Private Keys passphrase.

**Derivatives Trading Usds Futures**

#### REST API

- Modified response for `open_interest_statistics()` method (`GET /futures/data/openInterestHist`):
  - item property `CMCCirculatingSupply` added
- Fixed bug with duplicated `batchOrders` parameters.

## 6.0.0 - 2025-06-26

### Added (1)

- Added implementation of the `FromStr` trait for enums.

### Changed (10)

- Replaced the bounded broadcast channel in `WebsocketEventEmitter` with an unbounded channel.

**Spot**

#### REST API

- `RateLimits` is unified as a single object
- `ExchangeFilters` is unified as a single object
- Modified response for `exchange_info()` method (`GET /api/v3/exchangeInfo`):
  - `rate_limits`: item property `count` added
- Modified response for `order_cancel_replace()` method (`POST /api/v3/order/cancelReplace`):
  - property `newOrderResponse` added
  - property `newOrderResult` added
  - property `cancelResponse` added
  - property `cancelResult` added
  - `data`.`cancelResponse`: property `code` added
  - `data`.`cancelResponse`: property `msg` added
  - `data`.`newOrderResponse`: property `symbol` added
  - `data`.`newOrderResponse`: property `transactTime` added
  - `data`.`newOrderResponse`: property `clientOrderId` added
  - `data`.`newOrderResponse`: property `orderId` added
  - `data`.`newOrderResponse`: property `orderListId` added

- Modified response for `ticker()` method (`GET /api/v3/ticker`)
- Modified response for `ticker24hr()` method (`GET /api/v3/ticker/24hr`)
- Modified response for `ticker_trading_day()` method (`GET /api/v3/ticker/tradingDay`)

#### WebSocket API

- `RateLimits` is unified as a single object
- `ExchangeFilters` is unified as a single object
- Modified response for `exchange_info()` method (`POST /exchangeInfo`):
  - `rate_limits`: item property `count` added
  - `result`.`rate_limits`: item property `count` added

## 5.0.0 - 2025-06-24

### Changed (3)

- Fixed bug with Rest API signature param order.
- Using `rust_decimal::Decimal` for floating-point numbers.
- Modified response for `exchange_information()` method (`GET /fapi/v1/exchangeInfo`):
  - `assets`.`autoAssetExchange`: type `integer` → `string`
  - `symbols`.`filters`.`multiplierDecimal`: type `integer` → `string`

## 4.0.0 - 2025-06-20

### Changed (4)

- Made the `count` field required in `WebsocketApiRateLimit`.
- Corrected parameter naming to use camelCase instead of snake_case.
- Resolved floating-point precision issues.
- Fixed serialization of reserved keywords (e.g., `r#type`) so the `r#` prefix is no longer included.

## 3.0.0 - 2025-06-19

### Changed (2)

- Added `User-Agent` to `WebSocket` requests and distinguish them per module.
- Renamed enums following rust naming conventions.

## 2.0.1 - 2025-06-18

### Changed (1)

- Fix bug with multiple logger instances.

## 2.0.0 - 2025-06-17

### Added (1)

- `get_list_schedule()` (`GET /sapi/v1/margin/list-schedule`)

## 1.0.0 - 2025-06-12

- Initial release.
