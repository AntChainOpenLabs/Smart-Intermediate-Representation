// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0


use crate::context::Yul2IRContext;
use crate::yul;
use smart_ir::ir::printer::IRPrinter;
#[test]
fn yul_parser_tuple_test() {
    let expr = yul::ObjectParser::new().parse(
        r#"
        object "Token" {
            code {
                function selector_ret() -> s, z {
                    s := div(calldataload(0), 0x100000000000000000000000000000000000000000000000000000000)
                    z := div(calldataload(0), 0x100000000000000000000000000000000000000000000000000000000)
                }

                function selector_call() {
                    let a, b := selector_ret()
                }
            }
        }
        "#,
    ).unwrap();
    println!("{:?}", expr);
    let mut context = Yul2IRContext::new_with_object(expr);
    context.transform().unwrap();
    let mut p = IRPrinter::new(&context.ir_context);
    let mut w = String::new();
    p.print_modules(&mut w).unwrap();
    println!("{}", w);
}

#[test]
fn switch_stmt_test1() {
    let expr = yul::ObjectParser::new().parse(
        r#"
        object "Token" {
            code {
                switch selector()
                case 0x70a08231 {
                    returnUint(balanceOf(decodeAsAddress(0)))
                }
                case 0x18160ddd {
                    returnUint(totalSupply())
                }
                default {
                    revert(0, 0)
                }
            }
        }
        "#,
    ).unwrap();
    let mut context = Yul2IRContext::new_with_object(expr);
    context.transform().unwrap();
    let mut p = IRPrinter::new(&context.ir_context);
    let mut w = String::new();
    p.print_modules(&mut w).unwrap();
    println!("{}", w);
}

#[test]
fn switch_stmt_test2() {
    let expr = yul::ObjectParser::new().parse(
        r#"
        object "Token" {
            code {
                switch selector()
                case 0x70a08231 {
                    returnUint(balanceOf(decodeAsAddress(0)))
                }
                case 0x18160ddd {
                    returnUint(totalSupply())
                }
            }
        }
        "#,
    ).unwrap();
    let mut context = Yul2IRContext::new_with_object(expr);
    context.transform().unwrap();
    let mut p = IRPrinter::new(&context.ir_context);
    let mut w = String::new();
    p.print_modules(&mut w).unwrap();
    println!("{}", w);
}

#[test]
fn switch_stmt_test3() {
    let expr = yul::ObjectParser::new().parse(
        r#"
        object "Token" {
            code {
                switch selector()
                default {
                    revert(0, 0)
                }
            }
        }
        "#,
    ).unwrap();
    let mut context = Yul2IRContext::new_with_object(expr);
    context.transform().unwrap();
    let mut p = IRPrinter::new(&context.ir_context);
    let mut w = String::new();
    p.print_modules(&mut w).unwrap();
    println!("{}", w);
}


#[test]
fn yul_parser_erc20_test() {
    // erc20 yul ir
    // https://docs.soliditylang.org/en/latest/yul.html#complete-erc20-example
    let expr = yul::BlockParser::new().parse(
        r#"
        {
            sstore(0, caller())
            datacopy(0, dataoffset("runtime"), datasize("runtime"))
            return(0, datasize("runtime"))
        }  
        "#,
    );
    println!("{:?}", expr);

    let expr = yul::BlockParser::new().parse(
        r#"
        {
            require(iszero(callvalue()))

            switch selector()
            case 0x70a08231 {
                returnUint(balanceOf(decodeAsAddress(0)))
            }
            case 0x18160ddd {
                returnUint(totalSupply())
            }
            case 0xa9059cbb {
                transfer(decodeAsAddress(0), decodeAsUint(1))
                returnTrue()
            }
            case 0x23b872dd {
                transferFrom(decodeAsAddress(0), decodeAsAddress(1), decodeAsUint(2))
                returnTrue()
            }
            case 0x095ea7b3 {
                approve(decodeAsAddress(0), decodeAsUint(1))
                returnTrue()
            }
            case 0xdd62ed3e  {
                returnUint(allowance(decodeAsAddress(0), decodeAsAddress(1)))
            }
            case 0x40c10f19 {
                mint(decodeAsAddress(0), decodeAsUint(1))
                returnTrue()
            }
            default {
                revert(0, 0)
            }

            function mint(account, amount) {
                require(calledByOwner())
                mintTokens(amount)
                addToBalance(account, amount)
                emitTransfer(0, account, amount)
            }
            function transfer(to, amount) {
                executeTransfer(caller(), to, amount)
            }
            function approve(spender, amount) {
                revertIfZeroAddress(spender)
                setAllowance(caller(), spender, amount)
                emitApproval(caller(), spender, amount)
            }
            function transferFrom(from, to, amount) {
                decreaseAllowanceBy(from, caller(), amount)
                executeTransfer(from, to, amount)
            }

            function executeTransfer(from, to, amount) {
                revertIfZeroAddress(to)
                deductFromBalance(from, amount)
                addToBalance(to, amount)
                emitTransfer(from, to, amount)
            }

            function selector() -> s {
                s := div(calldataload(0), 0x100000000000000000000000000000000000000000000000000000000)
            }

            function decodeAsAddress(offset) -> v {
                v := decodeAsUint(offset)
                if iszero(iszero(and(v, not(0xffffffffffffffffffffffffffffffffffffffff)))) {
                    revert(0, 0)
                }
            }
            function decodeAsUint(offset) -> v {
                let pos := add(4, mul(offset, 0x20))
                if lt(calldatasize(), add(pos, 0x20)) {
                    revert(0, 0)
                }
                v := calldataload(pos)
            }

            function returnUint(v) {
                mstore(0, v)
                return(0, 0x20)
            }
            function returnTrue() {
                returnUint(1)
            }

            function emitTransfer(from, to, amount) {
                let signatureHash := 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
                emitEvent(signatureHash, from, to, amount)
            }
            function emitApproval(from, spender, amount) {
                let signatureHash := 0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925
                emitEvent(signatureHash, from, spender, amount)
            }
            function emitEvent(signatureHash, indexed1, indexed2, nonIndexed) {
                mstore(0, nonIndexed)
                log3(0, 0x20, signatureHash, indexed1, indexed2)
            }

            function ownerPos() -> p { p := 0 }
            function totalSupplyPos() -> p { p := 1 }
            function accountToStorageOffset(account) -> offset {
                offset := add(0x1000, account)
            }
            function allowanceStorageOffset(account, spender) -> offset {
                offset := accountToStorageOffset(account)
                mstore(0, offset)
                mstore(0x20, spender)
                offset := keccak256(0, 0x40)
            }

            function owner() -> o {
                o := sload(ownerPos())
            }
            function totalSupply() -> supply {
                supply := sload(totalSupplyPos())
            }
            function mintTokens(amount) {
                sstore(totalSupplyPos(), safeAdd(totalSupply(), amount))
            }
            function balanceOf(account) -> bal {
                bal := sload(accountToStorageOffset(account))
            }
            function addToBalance(account, amount) {
                let offset := accountToStorageOffset(account)
                sstore(offset, safeAdd(sload(offset), amount))
            }
            function deductFromBalance(account, amount) {
                let offset := accountToStorageOffset(account)
                let bal := sload(offset)
                require(lte(amount, bal))
                sstore(offset, sub(bal, amount))
            }
            function allowance(account, spender) -> amount {
                amount := sload(allowanceStorageOffset(account, spender))
            }
            function setAllowance(account, spender, amount) {
                sstore(allowanceStorageOffset(account, spender), amount)
            }
            function decreaseAllowanceBy(account, spender, amount) {
                let offset := allowanceStorageOffset(account, spender)
                let currentAllowance := sload(offset)
                require(lte(amount, currentAllowance))
                sstore(offset, sub(currentAllowance, amount))
            }

            function lte(a, b) -> r {
                r := iszero(gt(a, b))
            }
            function gte(a, b) -> r {
                r := iszero(lt(a, b))
            }
            function safeAdd(a, b) -> r {
                r := add(a, b)
                if or(lt(r, a), lt(r, b)) { revert(0, 0) }
            }
            function calledByOwner() -> cbo {
                cbo := eq(owner(), caller())
            }
            function revertIfZeroAddress(addr) {
                require(addr)
            }
            function require(condition) {
                if iszero(condition) { revert(0, 0) }
            }



        }  
        "#,
    );
    println!("{:?}", expr);
}

#[test]
fn yul2ir() {
    let block = yul::ObjectParser::new()
        .parse(
            r#"
            object "Token" {
                code {
                    sstore(0, caller())
            
                    datacopy(0, dataoffset("runtime"), datasize("runtime"))
                    return(0, datasize("runtime"))
                }
                object "runtime" {
                    code {
                        require(iszero(callvalue()))
            
                        switch selector()
                        case 0x70a08231 {
                            returnUint(balanceOf(decodeAsAddress(0)))
                        }
                        case 0x18160ddd {
                            returnUint(totalSupply())
                        }
                        case 0xa9059cbb {
                            transfer(decodeAsAddress(0), decodeAsUint(1))
                            returnTrue()
                        }
                        case 0x23b872dd {
                            transferFrom(decodeAsAddress(0), decodeAsAddress(1), decodeAsUint(2))
                            returnTrue()
                        }
                        case 0x095ea7b3 {
                            approve(decodeAsAddress(0), decodeAsUint(1))
                            returnTrue()
                        }
                        case 0xdd62ed3e {
                            returnUint(allowance(decodeAsAddress(0), decodeAsAddress(1)))
                        }
                        case 0x40c10f19 {
                            mint(decodeAsAddress(0), decodeAsUint(1))
                            returnTrue()
                        }
                        default {
                            revert(0, 0)
                        }
            
                        function mint(account, amount) {
                            require(calledByOwner())
            
                            mintTokens(amount)
                            addToBalance(account, amount)
                            emitTransfer(0, account, amount)
                        }
                        function transfer(to, amount) {
                            executeTransfer(caller(), to, amount)
                        }
                        function approve(spender, amount) {
                            revertIfZeroAddress(spender)
                            setAllowance(caller(), spender, amount)
                            emitApproval(caller(), spender, amount)
                        }
                        function transferFrom(from, to, amount) {
                            decreaseAllowanceBy(from, caller(), amount)
                            executeTransfer(from, to, amount)
                        }
            
                        function executeTransfer(from, to, amount) {
                            revertIfZeroAddress(to)
                            deductFromBalance(from, amount)
                            addToBalance(to, amount)
                            emitTransfer(from, to, amount)
                        }

                        function selector() -> s {
                            s := div(calldataload(0), 0x100000000000000000000000000000000000000000000000000000000)
                        }
            
                        function decodeAsAddress(offset) -> v {
                            v := decodeAsUint(offset)
                            if iszero(iszero(and(v, not(0xffffffffffffffffffffffffffffffffffffffff)))) {
                                revert(0, 0)
                            }
                        }
                        function decodeAsUint(offset) -> v {
                            let pos := add(4, mul(offset, 0x20))
                            if lt(calldatasize(), add(pos, 0x20)) {
                                revert(0, 0)
                            }
                            v := calldataload(pos)
                        }

                        function returnUint(v) {
                            mstore(0, v)
                            return(0, 0x20)
                        }
                        function returnTrue() {
                            returnUint(1)
                        }
            

                        function emitTransfer(from, to, amount) {
                            let signatureHash := 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
                            emitEvent(signatureHash, from, to, amount)
                        }
                        function emitApproval(from, spender, amount) {
                            let signatureHash := 0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925
                            emitEvent(signatureHash, from, spender, amount)
                        }
                        function emitEvent(signatureHash, indexed1, indexed2, nonIndexed) {
                            mstore(0, nonIndexed)
                            log3(0, 0x20, signatureHash, indexed1, indexed2)
                        }
            

                        function ownerPos() -> p { p := 0 }
                        function totalSupplyPos() -> p { p := 1 }
                        function accountToStorageOffset(account) -> offset {
                            offset := add(0x1000, account)
                        }
                        function allowanceStorageOffset(account, spender) -> offset {
                            offset := accountToStorageOffset(account)
                            mstore(0, offset)
                            mstore(0x20, spender)
                            offset := keccak256(0, 0x40)
                        }

                        function owner() -> o {
                            o := sload(ownerPos())
                        }
                        function totalSupply() -> supply {
                            supply := sload(totalSupplyPos())
                        }
                        function mintTokens(amount) {
                            sstore(totalSupplyPos(), safeAdd(totalSupply(), amount))
                        }
                        function balanceOf(account) -> bal {
                            bal := sload(accountToStorageOffset(account))
                        }
                        function addToBalance(account, amount) {
                            let offset := accountToStorageOffset(account)
                            sstore(offset, safeAdd(sload(offset), amount))
                        }
                        function deductFromBalance(account, amount) {
                            let offset := accountToStorageOffset(account)
                            let bal := sload(offset)
                            require(lte(amount, bal))
                            sstore(offset, sub(bal, amount))
                        }
                        function allowance(account, spender) -> amount {
                            amount := sload(allowanceStorageOffset(account, spender))
                        }
                        function setAllowance(account, spender, amount) {
                            sstore(allowanceStorageOffset(account, spender), amount)
                        }
                        function decreaseAllowanceBy(account, spender, amount) {
                            let offset := allowanceStorageOffset(account, spender)
                            let currentAllowance := sload(offset)
                            require(lte(amount, currentAllowance))
                            sstore(offset, sub(currentAllowance, amount))
                        }

                        function lte(a, b) -> r {
                            r := iszero(gt(a, b))
                        }
                        function gte(a, b) -> r {
                            r := iszero(lt(a, b))
                        }
                        function safeAdd(a, b) -> r {
                            r := add(a, b)
                            if or(lt(r, a), lt(r, b)) { revert(0, 0) }
                        }
                        function calledByOwner() -> cbo {
                            cbo := eq(owner(), caller())
                        }
                        function revertIfZeroAddress(addr) {
                            require(addr)
                        }
                        function require(condition) {
                            if iszero(condition) { revert(0, 0) }
                        }
                    }
                }
            }
        "#,
        )
        .unwrap();
    let mut context = Yul2IRContext::new_with_object(block);
    context.transform().unwrap();
    let mut p = IRPrinter::new(&context.ir_context);
    let mut w = String::new();
    p.print_modules(&mut w).unwrap();
    println!("{}", w);
}

#[test]
fn yul2ir_comment_test() {
    // build from
    // ```
    // // SPDX-License-Identifier: GPL-3.0
    // pragma solidity >=0.4.16 <0.9.0;
    //
    // contract SimpleStorage {
    //   uint storedData;
    //
    //   /// get somethings
    //   function set(uint x) public {
    //     storedData = x;
    //   }
    //
    //   function get() public view returns (uint) {
    //     return storedData;
    //   }
    // }
    // ```

    let block = yul::ObjectParser::new()
        .parse(
r#"
/*=====================================================*
 *                       WARNING                       *
 *  Solidity to Yul compilation is still EXPERIMENTAL  *
 *       It can result in LOSS OF FUNDS or worse       *
 *                !USE AT YOUR OWN RISK!               *
 *=====================================================*/


/// @use-src 0:"SimpleStorage.sol"
object "SimpleStorage_23" {
    code {
        /// @src 0:70:267  "contract SimpleStorage {..."
        mstore(64, memoryguard(128))
        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }

        constructor_SimpleStorage_23()

        let _1 := allocate_unbounded()
        codecopy(_1, dataoffset("SimpleStorage_23_deployed"), datasize("SimpleStorage_23_deployed"))

        return(_1, datasize("SimpleStorage_23_deployed"))

        function allocate_unbounded() -> memPtr {
            memPtr := mload(64)
        }

        function revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() {
            revert(0, 0)
        }

        /// @src 0:70:267  "contract SimpleStorage {..."
        function constructor_SimpleStorage_23() {

            /// @src 0:70:267  "contract SimpleStorage {..."

        }
        /// @src 0:70:267  "contract SimpleStorage {..."

    }
    /// @use-src 0:"SimpleStorage.sol"
    object "SimpleStorage_23_deployed" {
        code {
            /// @src 0:70:267  "contract SimpleStorage {..."
            mstore(64, memoryguard(128))

            if iszero(lt(calldatasize(), 4))
            {
                let selector := shift_right_224_unsigned(calldataload(0))
                switch selector

                case 0x60fe47b1
                {
                    // set(uint256)

                    if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                    let param_0 :=  abi_decode_tuple_t_uint256(4, calldatasize())
                    fun_set_14(param_0)
                    let memPos := allocate_unbounded()
                    let memEnd := abi_encode_tuple__to__fromStack(memPos  )
                    return(memPos, sub(memEnd, memPos))
                }

                case 0x6d4ce63c
                {
                    // get()

                    if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                    abi_decode_tuple_(4, calldatasize())
                    let ret_0 :=  fun_get_22()
                    let memPos := allocate_unbounded()
                    let memEnd := abi_encode_tuple_t_uint256__to_t_uint256__fromStack(memPos , ret_0)
                    return(memPos, sub(memEnd, memPos))
                }

                default {}
            }

            revert_error_42b3090547df1d2001c96683413b8cf91c1b902ef5e3cb8d9f6f304cf7446f74()

            function shift_right_224_unsigned(value) -> newValue {
                newValue :=

                shr(224, value)

            }

            function allocate_unbounded() -> memPtr {
                let a, b := allocate_unbounded()
                memPtr := mload(64)

            }

            function revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() {
                revert(0, 0)
            }

            function revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() {
                revert(0, 0)
            }

            function revert_error_c1322bf8034eace5e0b5c7295db60986aa89aae5e0ea0873e4689e076861a5db() {
                revert(0, 0)
            }

            function cleanup_t_uint256(value) -> cleaned {
                cleaned := value
            }

            function validator_revert_t_uint256(value) {
                if iszero(eq(value, cleanup_t_uint256(value))) { revert(0, 0) }
            }

            function abi_decode_t_uint256(offset, end) -> value {
                value := calldataload(offset)
                validator_revert_t_uint256(value)
            }

            function abi_decode_tuple_t_uint256(headStart, dataEnd) -> value0 {
                if slt(sub(dataEnd, headStart), 32) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := 0

                    value0 := abi_decode_t_uint256(add(headStart, offset), dataEnd)
                }

            }

            function abi_encode_tuple__to__fromStack(headStart ) -> tail {
                tail := add(headStart, 0)

            }

            function abi_decode_tuple_(headStart, dataEnd)   {
                if slt(sub(dataEnd, headStart), 0) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

            }

            function abi_encode_t_uint256_to_t_uint256_fromStack(value, pos) {
                mstore(pos, cleanup_t_uint256(value))
            }

            function abi_encode_tuple_t_uint256__to_t_uint256__fromStack(headStart , value0) -> tail {
                tail := add(headStart, 32)

                abi_encode_t_uint256_to_t_uint256_fromStack(value0,  add(headStart, 0))

            }

            function revert_error_42b3090547df1d2001c96683413b8cf91c1b902ef5e3cb8d9f6f304cf7446f74() {
                revert(0, 0)
            }

            function shift_left_0(value) -> newValue {
                newValue :=

                shl(0, value)

            }

            function update_byte_slice_32_shift_0(value, toInsert) -> result {
                let mask := 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
                toInsert := shift_left_0(toInsert)
                value := and(value, not(mask))
                result := or(value, and(toInsert, mask))
            }

            function identity(value) -> ret {
                ret := value
            }

            function convert_t_uint256_to_t_uint256(value) -> converted {
                converted := cleanup_t_uint256(identity(cleanup_t_uint256(value)))
            }

            function prepare_store_t_uint256(value) -> ret {
                ret := value
            }

            function update_storage_value_offset_0t_uint256_to_t_uint256(slot, value_0) {
                let convertedValue_0 := convert_t_uint256_to_t_uint256(value_0)
                sstore(slot, update_byte_slice_32_shift_0(sload(slot), prepare_store_t_uint256(convertedValue_0)))
            }

            /// @ast-id 14
            /// @src 0:138:191  "function set(uint x) public {..."
            function fun_set_14(var_x_6) {

                /// @src 0:185:186  "x"
                let _1 := var_x_6
                let expr_10 := _1
                /// @src 0:172:186  "storedData = x"
                update_storage_value_offset_0t_uint256_to_t_uint256(0x00, expr_10)
                let expr_11 := expr_10

            }
            /// @src 0:70:267  "contract SimpleStorage {..."

            function zero_value_for_split_t_uint256() -> ret {
                ret := 0
            }

            function shift_right_0_unsigned(value) -> newValue {
                newValue :=

                shr(0, value)

            }

            function cleanup_from_storage_t_uint256(value) -> cleaned {
                cleaned := value
            }

            function extract_from_storage_value_offset_0t_uint256(slot_value) -> value {
                value := cleanup_from_storage_t_uint256(shift_right_0_unsigned(slot_value))
            }

            function read_from_storage_split_offset_0_t_uint256(slot) -> value {
                value := extract_from_storage_value_offset_0t_uint256(sload(slot))

            }

            /// @ast-id 22
            /// @src 0:195:265  "function get() public view returns (uint) {..."
            function fun_get_22() -> var__17 {
                /// @src 0:231:235  "uint"
                let zero_t_uint256_2 := zero_value_for_split_t_uint256()
                var__17 := zero_t_uint256_2

                /// @src 0:250:260  "storedData"
                let _3 := read_from_storage_split_offset_0_t_uint256(0x00)
                let expr_19 := _3
                /// @src 0:243:260  "return storedData"
                var__17 := expr_19
                leave

            }
            /// @src 0:70:267  "contract SimpleStorage {..."

        }

        data ".metadata" hex"a3646970667358221220ca435a5e7194cf678ab6c844966938f10d8662d0c0de43a297699684a8a0191b6c6578706572696d656e74616cf564736f6c634300080b0041"
    }

}
"#,
        )
        .unwrap();
    // instr unimplement
    // let mut context = Yul2IRContext::new_with_object(block);
    // context.transform().unwrap();
    // let mut p = IRPrinter::new(&context.ir_context);
    // let mut w = String::new();
    // p.print_modules(&mut w).unwrap();
    // println!("{}", w);
}

#[test]
fn tuple_return_test() {
    let block = yul::ObjectParser::new()
        .parse(
r#"

/// @use-src 2:"contracts/Drop721.sol", 3:"contracts/ERC721A__OwnableUpgradeable.sol", 5:"erc721a-upgradeable/contracts/ERC721AUpgradeable.sol", 6:"erc721a-upgradeable/contracts/ERC721A__Initializable.sol", 8:"erc721a-upgradeable/contracts/IERC721AUpgradeable.sol"
object "Drop721_939" {
    code {
        /// @src 2:309:4277  "contract Drop721 is ERC721A__Initializable, ERC721AUpgradeable, ERC721A__OwnableUpgradeable {..."
        mstore(64, memoryguard(128))
        if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }

        constructor_Drop721_939()

        let _1 := allocate_unbounded()

        return(_1, datasize("Drop721_939_deployed"))
    }
    /// @use-src 2:"contracts/Drop721.sol", 3:"contracts/ERC721A__OwnableUpgradeable.sol", 4:"erc721a-upgradeable/contracts/ERC721AStorage.sol", 5:"erc721a-upgradeable/contracts/ERC721AUpgradeable.sol", 6:"erc721a-upgradeable/contracts/ERC721A__Initializable.sol", 7:"erc721a-upgradeable/contracts/ERC721A__InitializableStorage.sol"
    object "Drop721_939_deployed" {
        code {
            function abi_decode_tuple_t_addresst_address(headStart, dataEnd) -> value0, value1 {
                if slt(sub(dataEnd, headStart), 64) { revert_error_dbdddcbe895c83990c08b3492a0e83918d802a52331272ac6fdb6a7c4aea3b1b() }

                {

                    let offset := 0

                    value0 := abi_decode_t_address(add(headStart, offset), dataEnd)
                }

                {

                    let offset := 32

                    value1 := abi_decode_t_address(add(headStart, offset), dataEnd)
                }

            }

            function external_fun_isApprovedForAll_1898() {

                if callvalue() { revert_error_ca66f745a3ce8ff40e2ccaf1ad45db7774001b90d25810abd9040049be7bf4bb() }
                let param_0, param_1 :=  abi_decode_tuple_t_addresst_address(4, calldatasize())
                let ret_0 :=  fun_isApprovedForAll_1898(param_0, param_1)
                let memPos := allocate_unbounded()
                let memEnd := abi_encode_tuple_t_bool__to_t_bool__fromStack(memPos , ret_0)
                return(memPos, sub(memEnd, memPos))

            }
        }

        data ".metadata" hex"a26469706673582212205f10d436542158188120a80a7efd02df47fb581189ccbc579d8b5bdc5f2983d664736f6c63430008140033"
    }

}
"#,
        )
        .unwrap();
    // instr unimplement
    let mut context = Yul2IRContext::new_with_object(block);
    context.transform().unwrap();
    let mut p = IRPrinter::new(&context.ir_context);
    let mut w = String::new();
    p.print_modules(&mut w).unwrap();
    println!("{}", w);
}


