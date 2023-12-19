| Version | Editor | Desc. |
| --- | --- | --- |
| 1.0  | NeverRaR | Author |


## Informal Description of IR

In this section, we discuss various aspects of Smart IR (hereafter IR).

### A Simple Example
The top-level structure of IR is `contract`, `type`, `metadata` and the secondary structure is `state` and `fn`. The following example is a non-recursive Fibonacci sequence function written in IR:

```solidity
type struct.SimpleStorage.Foo = {a: u64，b: str}
contract SimpleStorage {
    state {foo : SimpleStorage.Foo，bar: i32}
  
    pub fn fib(%0: u64) -> u64 !ir_debug_location !0 {      
       0:
          let %1: u64 = 0: u64
          br_if(%0, bb 1, bb 2)
       1:
          %1 = add(%1, %0) 
          %0 = sub(%0, 1: u64) 
          br_if(%0, bb 1, bb 2)
       2: 
          ret(%1: u64)
      
    }
}

meta !0 = !{3: u32, 3: u32, "examples/hello_world.ir": str, }
```



### Syntax

IR currently contains only boolean, integer, and string literals. Since IR is based on CFG, in order to distinguish lable from symbolic names, in the readable version of IR we will add one `%` in front of variable names and one `@` in front of function names. In particular, The identifier in the IR may contain a symbol `.`.

- Literal：`true: bool`、 `0x1234: u128`、`189: i32`、`"abc"`
- Built-in instruction call: `add(2: u32, 3: u32, )`
- Variable declaration: 
   - Including initialization: `let %1: i32 = 2: i32`,  `let %2 : %SimpleStorage.Foo* = alloca(%SimpleStorage.Foo, )`
   - Default initialization (all 0 value):`let %1: i32`, `let %2: %SimpleStorage.Foo*`,`let %3: [%SimpleStorage.Foo*;50]`
- Assignment statement: `%1 = 2: u32`
- Control flow: 
   - Unconditional jump: `br(bb 1, )`
   - Conditional jump: `br_if(%0: bool, bb 1, bb 2, )` => `br_if(cond , if_bb , else_bb, )`
   - Match jump: `match(%0: i32, bb 1, 0: i32, bb 2, 1: i32, bb 3, 2: i32, bb 3, )` => `match(cond, default_bb , value1, bb1, value2, bb2, ... valueN, bbN, )`
- Function definition: `pub fn fib(%0: u64) -> u64 { ... }`
- Storage definition: `state {foo: SimpleStorage.Foo，bar: i32, }`
- Type definition: `type struct.SimpleStorage.Foo = {a: u64，b: str, }` 
- Field Access: 
   - `get_field(%0: %struct.SimpleStorage.Foo *, 1: i32, ) -> str`(get field) => `get_field(%0: %struct.T*, field_idx1: i32, field_idx2: i32 ... field_idxN: i32, ) -> fieldT`
   - `set_field(%0: %struct.SimpleStorage.Foo*,%value: str, 1: i32, )`(set field)=> `set_field(%0: %struct.T*, field_idx: i32, %field_value: FieldT, field_idx1: i32, field_idx2: i32 ... field_idxN: i32, )`
   - Specifically, for all `enum`, it has two fields, the first field is the index, and the second field is the current value.
      - `set_field(%0 : %enum.T*,enum_idx: i32,0: i32, )` and `get_field(%0 : %enum.T*,0: i32, ) -> i32` , set and get the index of `enum`
      - `set_field(%0 : %enum.T*, enum_value: valueT, 1: i32, )` and `get_field(%0 : %enum.T*,1: i32, ) -> valueT`, set and get the value of `enum`

### Type

- Primitive type: contains `bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128`. In addition, IR contains an empty type `void`. An empty type does not take up any memory.
- Pointer type: `T*`
- Special type: 
   - Storage path: `ir.builtin.StoragePath`
   - Fixed-length and unfixed-length array: `[T; N] 或 [T]`
   - Array iterator: `ir.vector.iter`
   - map: `{K: V}`
   - map iterator: `ir.map.iter`
- Custom Type: `type A = B` e.g. 
   - `type MyInt16 = i16` =>  `type A = B`
   - `type struct.MyInt256 = { high: i128, low: i128, }` => `type struct.A = { a: A, b: B ... }`
   - `type enum.I32OrStr = { a: i32, b: str, }` => `type enum.A = { a: A, b: B ... }`

### Literal

integer, string, boolean, void

### Function Call

- Normal call: use `call` instructions，e.g. `%4 = call(@ir.map.insert(%1: {str: i32}, %2: str, %3: i32, ), ) -> bool`
- Cross-contract call: Use `ir.builtin.co_call` intrinsic function where the corresponding contract address is `str` represented in the IR, e.g. `%4 = call(@ir.builtin.co_call(%contract_address: str, %method_name: str, %arg1: i32, %arg2: i64, ), ) -> i128`

### Control Flow

This section briefly introduces the use of IR control flow related instructions and how to convert the control flow of high-level language (rust as an example) code into IR.

#### if statement

```rust
{
    let a : u32 = 10;
    if a < 20 {
        a = 20;
    } 
    return a;
}
```

```rust
{
  0:
    let %0 :u32 = 10
    let %1 :bool = lt(%0 :u32, 20: u32, )
    br_if(%1 :bool, bb 1, bb 2, )
  1:
    %0 = 20
    br(bb 2, )
  2:
    ret(%0: u32, )
}
```

```rust
{
    let a : u32 = 10;
    if a < 20 {
        a = 20;
    } 
    return a;
}
```

```solidity
{
  0:
    let %0: u32 = 10
    let %1: bool = lt(%0: u32, 20: u32, )
    br_if(%1: bool, bb 1, bb 2, )
  1:
    %0 = 20: u32
    br(bb 5, )
  2:
    let %2: bool = lt(%0: u32, 30: u32, )
    br_if(%2: bool, bb 3, bb 4, )
  3:
    %0 = 30 :u32
    br(bb 5, )
  4:
    %0 = 40: u32
    br_if(bb 5, )
  5:
    ret(%0: u32, )
}
```



#### loop statement

```rust
{
    let v: u64 = 0;
    let a: u64 = 1;
    let b: u64 = 2;
    let i: u64 = 2;
    while i < 100 {
        v = a + b;
        b = a % 2;
        a = v % 2;
        i += 1;
    }
    return a;
}
```

```solidity
{
  0:
    let %0 :u64 = 0
    let %1 :u64 = 1
  	let %2 :u64 = 2
  	let %3 :u64 = 2
    br(bb 1, )
  1:
    let %4 :bool = lt(%3: u64, 100: u64)
    br_if(%4: bool, bb 2, bb 3)
  2:
    %0 = add(%1: u64, %2: u64, )
    %2 = mod(%1: u64, 2: u64, )
    %1 = mod(%0: u64, 2: u64, )
    %3 = add(%3: u64, 1: u64, )
    br(bb 1, )
  3:
    ret(%1: u64, )
}
```

```rust
struct ForContract {
}

impl ForContract {
    pub fn simple_vector(data: &mut Vec<i32>) -> i32 {
        data.push(3);
        data.push(4);
        data.push(5);
        let mut sum: i32 = 0;
        for (index, elem) in data.iter().enumerate() {
            sum = sum + index as i32 * elem;
        }
        return sum;
    }

    pub fn simple_map(data: &mut HashMap<String, u64>) -> u64 {
        data.insert("k2".to_string(), 20);
        data.insert("k3".to_string(), 30);
        let mut sum: u64 = 0;
        for (key, value) in data {
            sum = sum + *value;
        }
        return sum;
    }
}
```

```solidity
contract ForContract {
	pub fn simple_vector(%0: [i32], )-> i32 {
  	0:
    	call(@ir.vector.push(%0: [i32], 3: i32, ), )
    	call(@ir.vector.push(%0: [i32], 4: i32, ), )
    	call(@ir.vector.push(%0: [i32], 5: i32, ), )
    	let %1: u64 = 0: u64
    	let %2: %ir.vector.iter = call(@ir.vector.create_iter(%0: [i32], ) -> %ir.vector.iter, )
    	br(bb 1, )
  	1:
    	let %3: bool = call(@ir.vector.get_next(%0: [i32], %2: ir.vector.iter
    	                                 , false: bool, ) -> bool, ) 
    	br_if(%3: bool, bb 2,bb 3, )
  	2:
    	let %4: i32 = call(@ir.vector.obj_key(%2: %ir.vector.iter) -> i32, ) 
    	let %5: i32 = call(@ir.vector.obj_value(%2: %ir.vector.iter)  -> i32, )
    	let %6: i32 = mul(%4: i32, %5: i32, )
    	%3 = add(%3: i32, %6: i32, )
    	br(bb 1, )
  	3:
    	ret(%1: u64, )
	}

	pub fn simple_map(%0: {str: u64}, )-> u64 {
  	0:
    	call(@ir.map.insert(%0: {str: u64}, "k2": str, 20: u64, ), )
    	call(@ir.map.insert(%0: {str: u64}, "k3": str, 30: u64, ), )
    	let %1: u64 = 0: u64
    	let %2: %ir.map.iter = call(@ir.map.create_iter(%0: {str: u64}, ), ) -> %ir.map.iter
    	br(bb 1, )
  	1:
    	let %3: %ir.map.iter = call(@ir.map.get_next(%0: {str: u64}, %2: %ir.map.iter
                                     , false: bool, ) -> bool, ) 
    	br_if(%3: bool, bb 2,bb 3, )
  	2:
    	let %4: str = call(@ir.map.obj_key(%2: ir.map.iter, ) -> str, ) 
    	let %5: u64 = call(@ir.map.obj_value(%2: ir.map.iter, ) -> u64, )
    	%3 = add(%3: i32, %5: i32, )
    	br(bb 1, )
  	3:
    	ret(%1: u64, )
	}
}
```



#### match statement

```rust
// This is a normal enum case
enum Color {
    Red,
    Yellow,
    Blue,
}

struct Dog {
    color: str,
}
struct Cat {
    color: str,
}
struct Rabbit {
    size: u32,
}

// This is a enum case with the internal struct definition
enum Animal {
    Dog(Dog),
    Cat(Cat),
    Rabbit(Rabbit),
}

struct MatchContract {}

impl MatchContract {
    pub fn simple_match() {
        let color: Color = Color::Red;
        match color {
            Color::Red => {}
            Color::Yellow => {}
            Color::Blue => {}
        }
        match color {
            Color::Red => {}
            _ => {} // The default case
        }
        let animal: Animal = Animal::Dog(Dog { color: "red" });
        match animal {
            Animal::Dog(dog) => {}
            _ => {}
        }
    }
}
```

```solidity
type enum.MatchContract.Color = {Red: void, Yellow: void, Blue: void, }
type enum.MatchContract.Animal = {Dog: struct.MatchContract.Dog*, 
                                  Cat: struct.MatchContract.Cat*,
                                  Rabbit: struct.MatchContract.Rabbit*, }
type struct.MatchContract.Dog = {color: str, }
type struct.MatchContract.Cat = {color: str, }
type struct.MatchContract.Rabbit = {size: u32, }
contract MatchContract {
	pub fn simple_match() {
  	0:
      let %0: %enum.MatchContract.Color*  = alloc(%enum.MatchContract.Color, )
      set_field(%0: %enum.MatchContract.Color*, 0: i32, 0: i32, )
      let %1: i32 = get_field(%0: %enum.MatchContract.Color*, 0: i32, ) -> i32
      match(%1: i32, bb 1, 0: i32, bb 1, 1: i32, bb 2, 2: i32, bb 3, )
    1:
      br(bb 4, )
    2:
      br(bb 4, )
    3:
      br(bb 4, )
  	4:
     match(%1: i32, bb 5, 0: i32, bb 6, )
    5:
      br(bb 7, )
    6:
      br(bb 7, )
    7:
      let %2: %enum.MatchContract.Animal* = alloc(%enum.MatchContract.Animal, )
      let %3:  = alloc(%struct.MatchContract.Dog, )
      set_field(%3: %struct.MatchContract.Dog*, 0: i32,"red": str, )
      set_field(%2: %enum.MatchContract.Animal*, 0: i32, 0: i32, )
      set_field(%2: %enum.MatchContract.Animal*, 1: i32, 
                %3: %struct.MatchContract.Dog*, )
      match(%1: i32, bb 8, 0: i32, bb 9, )
    8:
      br(bb 10, )
    9:
      let %4 = get_field(%3: %enum.MatchContract.Animal*, 1: i32, ) 
            -> %struct.MatchContract.Dog*
      br(bb 10, )
    10:
      ret()
  }
  
}
```



### 



### Type conversion

IR only supports integer type conversion, using instructions `int_cast`, e.g. `int_cast(0: i32) -> u128`

### Storage Access

The main instructions involved in memory access are as follows:

-  `get_storage_path` instruction，e.g. `%2 = get_storage_path(%1: %ir.builtin.StoragePath, "hello": str, "world": str, 1: u32, %5: str, )`,  parameter type can be string, integer, and `ir.builtin.StoragePath`，return an abstract structure. `ir.builtin.StoragePath` 
-  `storage_load` instruction，e.g. `%3 = storage_load(%2: %ir.builtin.StoragePath) -> i32` 
-  `storage_store`instruction, e.g. `storage_store(%2: %ir.builtin.StoragePath,%3: i32)`

```solidity
contract SimpleStorage {
  
	state {
      vec: [u64]
      a: u64
  }
  
	fn st_test(%0: ir.builtin.StoragePath, ) -> u64 {
    0:
    let %1: %ir.builtin.StoragePath = 
    			get_storage_path(%0: ir.builtin.StoragePath, 0: i32, ) 
    let %2: u64 = storage_load(%1: ir.builtin.StoragePath, ) -> u64
    ret(%2, )
  }

  pub fn set() -> u64 {
    0:
      let %0: %ir.builtin.StoragePath = get_storage_path("vec": str, 0: i32, )
      storage_store(%0: ir.builtin.StoragePath, 10: u64, )
      let %2:  = get_storage_path("vec": str, )
      let %3 = call(@st_test(%2) -> u64, )
      ret(%3, )
  }
  
}
```

```solidity
contract SimpleStorage {
    state {
        storedData: [u64]
    }

    pub fn test_push(%0: u64) {
        let %1: %ir.builtin.StoragePath = get_storage_path("storedData": str, ) 
        call(@ir.storage.array_push(%1: ir.builtin.StoragePath, 
                                       %0: u64, ))
    }

    pub fn get(%0: i32) -> u64 {
        let %1: %ir.builtin.StoragePath = get_storage_path("storedData": str, %0: i32)
        let %2 : u64 = storage_load(%1: ir.builtin.StoragePath, ) -> u64
        ret(%2, )
    }
   
    pub fn test_pop() {
        let %1: %ir.builtin.StoragePath = get_storage_path("storedData": str, ) 
        call(@ir.storage.array_pop(%1: ir.builtin.StoragePath, ), )
    }
}
```



### IntrinsicFunctions

IR supports the concept of "IntrinsicFuntion", and all IntrinsicFuntion names must start with the prefix "ir.". This prefix is reserved for internal names in IR; Therefore, function names cannot start with this prefix. IntrinsicFuntion must always be an external function: users cannot define the body of IntrinsicFuntion. Intrinsic functions can only be used in calls or calling instructions. Additionally, since IntrinsicFuntion is a part of the IR language, if any intrinsic functions are added, they need to be recorded in the Spec.

Some IntrinsicFuntons can be overloaded, meaning that IntrinsicFuntons represent a series of functions that perform the same operation but for different data types.

### IR Expansion

#### Metadata
Metadata is used to record information that is unique to the AST and affects IR lowering. It can be attached to type declarations, instructions, and functions. Take `ir_debug_location` as an example

```solidity
type struct.MatchContract.Dog = {color: str, } !ir_debug_location !0 
type struct.MatchContract.Cat = {color: str, }  !ir_debug_location !1 
type struct.MatchContract.Rabbit = {size: u32, } !ir_debug_location !2 

meta !0 = !{3: u32, 5: u32, "examples/hello_world.ir": str, }
meta !1 = !{4: u32, 5: u32, "examples/hello_world.ir": str, }
meta !2 = !{5: u32, 3: u32, "examples/hello_world.ir": str, }
```
## IR Specification

### Instruction Specification
This section contains all the directives of the IR and their corresponding specifications.
#### Control Flow Instruction
All basic blocks in the IR should end with a control flow indicating which basic block should be executed next after the execution of the current block is complete. Current control flow instructions include:`br`、 `br_if``、`match`、`ret`
##### ret
**Grammar**
```solidity
ret(<value> : <type>, )       //Return a value from a non-void function
ret()                       //Return from void function
```
**Overview**
`ret`  is used to return control flow (and optionally a value) from a function to the caller. 
`ret`  come in two forms: those that return a value and then transfer control flow, and those that simply cause control flow transfers.
**Parameters**
`ret` takes an optional single argument: the return value of the current function.
**Semantics**
When `ret` instruction is  executed ,the control flow returns to the context of the calling function and continues execution of the remaining instructions at the `call` instruction. 
**Examples**
```solidity
ret(5: i32, )             // Return an integer value of 5
ret()                   // Return from a void function
ret("hello": str, )       // Return a string  alue of "hello"
```
##### br
**Grammar**
```solidity
br(bb <target>, ) 
```
**Overview**
`br` is used to unconditionally transfer control flow to a different basic block in the current function. 
**Parameters**
`br`  takes a single argument:  the target basic block
**Semantics**
When `br` instruction is executed, the control flow transfers to the beginning of the target basic block. 
**Examples**
```solidity
0:
  br(bb 1, )
1:
	...
```
##### br_if
**Grammar**
```solidity
br_if(<cond>: bool, bb <if_true>, bb <if_false>, ) 
```
**Overview**
`br_if`  is used to  conditionally transfer control flow to different basic blocks in the current function.
**Parameters**
`br_if` accepts three parameters: a jump condition and two jump targets. 
**Semantics**
When `br_if` instruction is executed，the `bool` argument is evaluated.f the value is true, the contral flow transfer to `bb <if_true>`. If the value is false, the contral flow transfer to`bb <if_false>` .
**Examples**
```solidity
Test:
  %cond = eq(%a: i32, %b: i32, )
  br_if(%cond: bool, bb IfEqual, bb IfUnequal, )
IfEqual:
  ret(1: i32, )
IfUnequal:
  ret(0: i32, )
```
##### match
**Grammar**
```solidity
match(<value>: <int_ty>, bb <default>, <val1>: <int_ty>, bb <target1>,... ) 
```
**Overview**
`match` is used to transfer control flow to one of several different locations. It is a generalization of the `br_if` instruction that allows the branch to occur to one of many possible destinations.
**Parameters**
`match` accepts three parameters: a comparison value `<value>` of integer type, a default jump basic block `<default>`, and an array containing pairs of comparison value constants and corresponding jump basic blocks, where the comparison value constants do not allow repetition.
**Semantics**
`match` specifies a value and a jump target table. When the `match` instruction executes, the table is searched for the given value. If the value is found, control flow is transferred to the corresponding basic block; otherwise, control flow is transferred to the default basic block.
**Examples**
```solidity

match(%val: i32, bb truedest, 0: i32,bb falsedest, )

match(0: i32, bb dest, )

match(%val: i32, bb otherwise, 0: i32, bb onzero,
                               1: i32 bb onone,
                               2: i32, bb ontwo, )
```
#### Unary Instruction
Unary instruction takes a single operand, performs an operation on it, and produces a single value of the same type as the operand.
##### not
**Grammar**
```solidity
<result> = not(<op1> : bool, )
```
**Overview**
`not`  returns the inverse of the operand.
**Parameters**
`not` takes one argument:  an  `bool` operand 
**Semantics**
produces a boolean value that is the inverse of the operand.
**Examples**
```solidity
%result = not(%val: bool, )  //result = !val
```
##### bit_not
**Grammar**
```solidity
<result> = bit_not(<op1>: <int_ty>, )
```
**Overview**
`bit_not`  returns the bitwise-inverted result of the operand. 
**Parameters**
`bit_not`takes one argument: an integer operand 
**Semantics**
produces an integer value with the operand bitwise-inverted.
**Examples**
```solidity
%result = bit_not(%val: i32, )  //result = ~val
```
#### Binary Instruction
    Binary operators are used to perform most of the calculations in a program. They require two operands of the same type, perform an operation on them, and produce a value that has the same type as their operands.
##### add
**Grammar**
```solidity
<result> = add(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`add`  returns the sum of its two operands.
**Parameters**
`add`  takes two arguments, both operands must be integers of the same type,
**Semantics**
produces the sum of the two operands.
**Examples**
```solidity
%result = add(1: i32, 2: i32, )  //result = 1 + 2
```
##### sub
**Grammar**
```solidity
<result> = sub(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview **
`sub` returns the difference of its two operands.
**Parameters**
`sub`  takes two arguments, both operands must be integers with the same type
**Semantics**
produces the difference of the two operands.
**Examples**
```solidity
%result = sub(1: i32, 2: i32, )  //result = 1 -2
```
##### mul
**Grammar**
```solidity
<result> = mul(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`mul`  returns the product of its two operands. 
**Parameters**
`mul`  takes two arguments, both operands must be integers with the same type
**Semantics**
produces the product of the two operands.
**Examples**
```solidity
%result = mul(1: i32, 2: i32, ) //result = 1*2
```
##### div
**Grammar**
```solidity
<result> = div(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`div` returns the quotient of its two operands.
**Parameters**
`div`takes two arguments, both operands must be integers  with the same type
**Semantics**
produces the quotient of the two operands.
**Examples**
```solidity
%result = div(3: i32, 2: i32, ) //result = 3/2 = 1
```
#####  mod
**Grammar**
```solidity
<result> = mod(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`mod` returns the remainder of an integer division of its two operands. 
**Parameters**
`mod`takes two arguments, both operands must be integers with the same type
**Semantics**
produces the remainder of an integer division of the two operands.
**Examples**
```solidity
%result = mod(3: i32, 2: i32, ) //result = 3%2 = 1
```
##### exp
**Grammar**
```solidity
<result> = exp(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`exp`  returns the specified integer raised to the power  of an integer.
**Parameters**
`exp` takes two arguments: base and exponent 
**Semantics**
produces the base raised to the power of the exponent 
**Examples**
```solidity
%result = exp(3: i32, 2: i32, ) //result = 3**2 = 9
```
##### and
**Grammar**
```solidity
<result> = and(<op1>: bool, <op2>: bool, )
```
**Overview**
`and` returns the and of boolean operands
**Parameters**
`and`takes two boolean operands
**Semantics**
produces the and of the two boolean operands.
**Examples**
```solidity
%result = and(true: bool, false: bool, ) //result = true && false = false
```
##### bit_and
**Grammar**
```solidity
<result> = bit_and(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`bit_and` returns the result of the bitwise AND of two operands
**Parameters**
`bit_and`takes two arguments, both operands must be integers with the same type
**Semantics**
produces the result of the bitwise AND of two operands
**Examples**
```solidity
%result = bit_and(1: i32, 2: i32, ) //result = 1 & 2
```
##### or
**Grammar**
```solidity
<result> = or(<op1>: bool, <op2>: bool, )
```
**Overview**
`or` returns the or of two Boolean operands
**Parameters**
`or`takes two boolean operands
**Semantics**
produces the or of the two boolean operands.
**Examples**
```solidity
%result = or(true: bool, false: bool, ) //result = true || false = true
```
##### bit_or
**Grammar**
```solidity
<result> = bit_or(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`bit_or`  returns the result of the bitwise OR of two operands
**Parameters**
`bit_or`takes two arguments, both operands must be integers with the same type
**Semantics**
produces the result of the bitwise OR of two operands
**Examples**
```solidity
%result = bit_or(1: i32, 2: i32, ) //result = 1 | 2
```
##### bit_xor
**Grammar**
```solidity
<result> = bit_xor(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`bit_xor` returns the result of the bitwise XOR of two operands
**Parameters**
`bit_xor`takes two arguments, both operands must be integers with the same type
**Semantics**
produces the result of the bitwise OR of two operands
**Examples**
```solidity
%result = bit_xor(1: i32, 2: i32, ) //result = 1 ^ 2
```
##### shl
**Grammar**
```solidity
<result> = shl(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`shl` returns the first operand shifted left by the specified number of bits. 
**Parameters**
`shl`takes two arguments, both operands must be integers with the same type
**Semantics**
produces ![](https://intranetproxy.alipay.com/skylark/lark/__latex/dd577175599b979d32fe79efc691e126.svg#card=math&code=op1%2A2%5E%7Bop2%7D%7B%5C%2C%7Dmod%7B%5C%2C%7D%202%5E%7Bn%7D&id=FHm9V), where n is the width of the result
**Examples**
```solidity
%result = shl(1: i32, 2: i32, ) //result = 1 << 2
```

##### shr
**Grammar**
```solidity
<result> = shr(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`shr` returns the first operand logically shifted right by the specified number of bits. 
**Parameters**
`shr` takes two arguments, both operands must be integers with the same type
**Semantics**
`shr` performs a logical shift right, and the most significant bit of the result is padded with zero bits after the shift.
**Examples**
```solidity
%result = shr(8: i32, 2: i32, ) //result = 8 >> 2
```
##### sar
**Grammar**
```solidity
<result> = sar(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`sar` returns the result of shifting the first operand to the right by a specified number of bits
**Parameters**
`sar`takes two arguments, both operands must be integers with the same type
**Semantics**
 `sar`performs an arithmetic shift right. The most significant bit of the result is shifted and then padded with the sign bit.
**Examples**
```solidity
%result = sar(8: i32, 2: i32, ) //result = 8 >>> 2
```

#### Compare Instruction
    Comparison instructions are used to perform most conditional calculations in a program. They require two operands of the same type, perform an operation on them, and produce a boolean value.
##### eq
**Grammar**
```solidity
<result> = eq(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`eq` compares two integer operands for equality
**Parameters**
`eq`takes two arguments, both operands must be integers with the same type
**Semantics**
compares the values of two integer operands and produces true if they are equal. Otherwise, it produces false
**Examples**
```solidity
%result = eq(8: i32, 2: i32, ) //result = 8==2 = false
```
##### ne
**Grammar**
```solidity
<result> = ne(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`ne` compares whether two integer operands are not equal
**Parameters**
`ne`takes two arguments, both operands must be integers with the same type
**Semantics**
compares the values of two integer operands and produces true if they are not equal. Otherwise, it produces false
**Examples**
```solidity
%result = ne(8: i32, 2: i32, ) //result = 8!=2 = true
```
##### gt
**Grammar**
```solidity
<result> = gt(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`gt` compares whether the first operand is greater than the second operand 
**Parameters**
`gt`takes two arguments, both operands must be integers with the same type
**Semantics**
compares the values of two integer operands and produces true if the first operand is greater than the second.Otherwise, it produces false.
**Examples**
```solidity
%result = gt(8: i32, 2: i32, ) //result = 8 > 2 = true
```
##### ge
**Grammar**
```solidity
<result> = ge(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`ge` compares whether the first operand is greater than or equal to the second operand 
**Parameters**
`ge`takes two arguments, both operands must be integers with the same type
**Semantics**
compares the values of two integer operands and produces true if the first operand is  greater than or equal to the second.Otherwise, it produces false.
**Examples**
```solidity
%result = gt(8: i32, 2: i32, ) //result = 8 >= 2 = true
```

##### lt
**Grammar**
```solidity
<result> = lt(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`lt` compares whether the first operand is less than the second operand 
**Parameters**
`lt` takes two arguments, both operands must be integers with the same type
**Semantics**
compares the values of two integer operands and produces true if the first operand is  less than the second. Otherwise, it produces false.
**Examples**
```solidity
%result = gt(8: i32, 2: i32, ) //result = 8 < 2 = false
```
##### le
**Grammar**
```solidity
<result> = le(<op1>: <int_ty>, <op2>: <int_ty>, )
```
**Overview**
`le` compares whether the first operand is less than or equal to the second operand 
**Parameters**
`le`takes two arguments, both operands must be integers with the same type
**Semantics**
compares the values of two integer operands and produces true if the first operand is  less than or equal to the second.Otherwise, it produces false.
**Examples**
```solidity
%result = le(8: i32, 2: i32, ) //result = 8 <= 2 = false
```
#### Memory Instruction
Memory instructions are responsible for allocating and freeing memory, as well as accessing pointers. IR only supports the allocation of struct pointers and field access.
##### alloca
**Grammar**
```solidity
<result> = alloca(<ty>, )
```
**Overview**
`alloca` allocates memory on the stack frame of the currently executing function and releases it automatically when the function returns to its caller.
**Parameters**
`alloca`takes only one struct argument ( `struct` or `enum`), allocates sizeof(<ty>) bytes of memory on the runtime stack, and returns a pointer of the appropriate type to the program. 
**Semantics**
Allocated memory is uninitialized. Loading from uninitialized memory results in an undefined value. Memory allocated by `alloca`  is automatically released when the function returns. 
**Examples**
```solidity
%result = alloca(%struct.SimpleStorage.Foo, )
```
##### malloc
**Grammar**
```solidity
<result> = malloc(<ty>, )
```
**Overview**
`malloc` allocates memory on the memory area, and the memory is not automatically released. 
**Parameters**
`malloc`takes only one struct argument ( `struct` or `enum`), allocates sizeof (<ty>) bytes of memory on the run-time memory area, and returns a pointer of the appropriate type to the program. 
**Semantics**
Allocated memory is uninitialized. Loading from uninitialized memory results in an undefined value. Memory allocated by `malloc` is not automatically released when the function returns. 
**Examples**
```solidity
%result = alloc(%struct.SimpleStorage.Foo, )
```
##### free
**Grammar**
```solidity
free(<ptr>: <ty>*, )
```
**Overview**
`free` releases  the memory  allocated by `malloc`.
**Parameters**
`free`only accepts one pointer parameter
**Semantics**
`free` releases the memory allocated by  `malloc` . Releasing memory allocated by `alloca`  or releasing memory that has already been freed again is an undefined behavior.
**Examples**
```solidity
free(%2 :%struct.SimpleStorage.Foo*, )
```

##### get_field
**Grammar**
```solidity
<result> = get_field(<ptr>: <ty>*, <field_idx1>: i32...) -> <field_ty>
```
**Overview**
`get_field` gets the value of the specified field of the struct. 
**Parameters**
`get_field`accepts a struct pointer parameter and several index integer parameters 
**Semantics**
`get_field`reads the value from the struct pointer according to the index parameter, and the index starts from 0. Note that if a field is a struct and the field is not initialized, direct access to an internal field of the field through a multi-level index is undefined behavior. 
**Examples**
```solidity
type struct.SimpleStorage.Foo = {a: u8, bar : struct.SimpleStorage.Bar*, b: u32, }
type struct.SimpleStorage.Bar = {a: u16, b: str, }
%result = get_field(%0: %struct.SimpleStorage.Foo*,1: i32,1: i32, ) -> str
// result = %0.bar.b
```

##### set_field
**Grammar**
```solidity
set_field(<ptr>: <ty>*, <field_value>: <field_ty>, <field_idx1>: i32...)
```
**Overview**
`set_field` sets the value of the specified field of the struct. 
**Parameters**
`set_field`accepts a struct pointer parameter, a field value parameter, and several index integer parameters.
**Semantics**
`set_field` writes the value from the struct pointer according to the index parameter, and the index starts from 0. Note that if a field is a struct and the field is not initialized, direct access to an internal field of the field through a multi-level index is undefined behavior. 
**Examples**
```solidity
type struct.SimpleStorage.Foo = {a: u8, bar : struct.SimpleStorage.Bar*, b: u32, }
type struct.SimpleStorage.Bar = {a: u16, b: str, }

set_field(%0: %struct.SimpleStorage.Foo*,"hello": str, 1: i32,1: i32, )
// %0.bar.b = "hello"
```
#### Storage Instruction
Storage instructions are responsible for accessing and modifying storage.
#####  get_storage_path
**Grammar**
```solidity

// path_ty = str |  int_ty | ir.builtin.StoragePath

get_storage_path( <path1> : <path_ty> ...)
```
**Overview**
`get_storage_path` combines the storage access parameters to generate a complete storage access path 
**Parameters**
`get_storage_path` accepts several path parameters, and the type of the path parameter can be `str` ,integer and `ir.builtin.StoragePath`
**Semantics**
`get_storage_path` splices the path parameter to generate one `ir.builtin.StoragePath`. It is an abstract type  of storage access path
**Examples**
```solidity
%2 = get_storage_path( "hello": str, %1: i32, )
```
#####  storage_load
**Grammar**
```solidity
storage_load(<path> : ir.builtin.StoragePath) -> <load_ty>
```
**Overview**
`storage_load` accesses the storage according to the storage path and reads out the specified type
**Parameters**
`storage_load`accepts a storage path parameter with the type `ir.builtin.StoragePath`
**Semantics**
`storage_load` produces a value read from the storage. The value type is specified by the instruction
**Examples**
```solidity
%2 = storage_load(%0: %ir.builtin.StoragePath, ) -> i32
```
#####  
##### storage_store
**Grammar**
```solidity
storage_store(<path> : %ir.builtin.StoragePath, <value> : <store_ty>, ) 
```
**Overview**
`storage_store` accesses the storage according to the storage path, and writes a value with the specified type. 
**Parameters**
`storage_store`ccepts two parameters, one is the storage path parameter, the type is `ir.builtin.StoragePath`, and the other is the value to be written.
**Semantics**
`storage_store`writes a value to the storage. the type of value is specified by the parameter 
**Examples**
```solidity
%2 = storage_store(%0: %ir.builtin.StoragePath, 10: i32, )
```
#### Other Instruction
#####  call
**Grammar**
```solidity
<result> = call(@<func_name>(<arg1> : <arg_ty1>...) -> <ret_ty>, ) 
```
**Overview**
`call` represents a simple function call
**Parameters**
`call`takes a function name and the parameters corresponding to the function call as parameters 
**Semantics**
`call`is used to transfer control flow to the specified function, and its incoming parameters are bound to the specified values. According to the `ret` instructions in the called function, the control flow continues to execute the instructions after the function call, and the return value of the function is bound to the result parameter.
**Examples**
```solidity
%result = call(@ir.context.call.sender()  -> str, )
```

#####  int_cast
**Grammar**
```solidity
<result> = int_cast(<value>: <int_ty>, ) -> <int_ty>
```
**Overview**
`int_cast` indicates an integer type conversion 
**Parameters**
`int_cast`accepts an integer parameter 
**Semantics**
`int_cast`is used to perform a type conversion between integer types. The type resulting from the conversion is specified by the instruction. 
**Examples**
```solidity
%result = int_cast(0: i32, ) -> i128
```

### IntrinsicFunctions
#### vector
```solidity
//Get specified elements from an array
func ir.vector.get(%array : [<ty>], %idx : i32, ) -> <val_ty>

//Set the value of a certain position in the array
func ir.vector.set(%array : [<ty>], %idx : i32, %val: <val_ty>, ) 

//Insert value to the end of the array
func ir.vector.push(%array : [<ty>], %val: <val_ty>, ) 

//Insert a value into a certain position in the array
func ir.vector.insert(%array : [<ty>], %idx : i32, %val: <val_ty>, ) 

//Delete value at a certain position in the array
func ir.vector.delete(%array : [<ty>], %idx : i32, ) 

//Get the length of the array
func ir.vector.len(%array : [<ty>], ) -> i32

//Clear the array
func ir.vector.clear(%array : [<ty>], )

//Reverse the array
func ir.vector.reverse(%array : [<ty>], )

//Get a copy of the array slice
func ir.vector.slice(%array : [<ty>],%begin: i32, %end: i32, ) -> [<ty>]

//Convert array into strings by byte
func ir.vector.to_str(%array : [<ty>]) -> str

//Create an array iterator
func ir.vector.create_iter(%array: [<ty>], ) -> %ir.vector.iter

//Get the next iterator
func ir.vector.get_next(%iter: %ir.vector.iter, ) -> bool

//Get the corresponding index of the iterator
func ir.vector.obj_key(%iter: %ir.vector.iter, ) -> i32

//Get the corresponding value of the iterator
func ir.vector.obj_value(%iter: %ir.vector.iter, ) -> <val_ty>
```
#### map
```solidity
//Get specified elements from the map
func ir.map.get(%map: {<key_ty>: <val_ty>}, %key: <key_ty>, ) -> <val_ty> 

//Set specified elements from the map
func ir.map.set(%map: {<key_ty>: <val_ty>}, %key: <key_ty>, %val: <val_ty>, ) 

//Check if the specified key exists in the map
func ir.map.contains_key(%map: {<key_ty>: <val_ty>}, %key: <key_ty>, ) -> bool

//Delete value from the map according the key
func ir.map.delete(%map: {<key_ty>: <val_ty>}, %key: <key_ty>, )

//Get the length of the map
func ir.map.len(%map: {<key_ty>: <val_ty>}, )-> i32

//Clear the map
func ir.map.clear(%map: {<key_ty>: <val_ty>}, )

//Create a map iterator
func ir.map.create_iter(%array: {<key_ty>: <val_ty>}, ) -> %ir.map.iter

//Get the next iterator
func ir.map.get_next(%iter: %ir.map.iter, ) -> bool

//Get the corresponding key of the iterator
func ir.map.obj_key(%iter: %ir.map.iter, ) -> <key_ty>

//Get the corresponding value of the iterator
func ir.map.obj_value(%iter: %ir.vector.iter, ) -> <val_ty>
```

#### storage
```solidity
//Pushing data into an array in storage
func ir.storage.push(%path: %ir.builtin.StoragePath, %val: <val_ty>, )

//Pushing empty data into an array in storage ,
// which will increase the size of the array in storage by 1
func ir.storage.push_empty(%path: %ir.builtin.StoragePath, )

//Pop data from an array in storage
func ir.storage.pop(%path: %ir.builtin.StoragePath, )

//Get the length of the array in storage
func ir.storage.len(%path: %ir.builtin.StoragePath, ) -> i32

//Check if the stored map contains the corresponding key
func ir.storage.contains_key(%path: %ir.builtin.StoragePath,%key: <key_ty>, ) 
							-> bool

//Delete value from storage
func ir.storage.delete(%path: %ir.builtin.StoragePath, )

//Connecting two storage paths
func ir.storage.join(%path: %ir.builtin.StoragePath, 
                     %remain: %ir.builtin.StoragePath, )
                               -> %ir.builtin.StoragePath
```

#### str
```solidity
//Get the length of the string
func ir.str.len(%s: str, ) -> i32

//Get character at specific positions in a string
func ir.str.at(%s: str, %index: i32 , ) -> u8

//Get the copy after converting the string to lowercase
func ir.str.lower(%s: str, ) -> str

//Get the copy after converting the string to uppercase
func ir.str.upper(%s: str, ) -> str

//Returns the number of times a substring appears in the current string
func ir.str.count(%s: str, %sub: str) -> u32


//Use %sep as a delimiter to split %s and return the split string array
//%sep - delimited string
//%maxsplit - maximum number of divisions, -1 indicates no maximum limit
func ir.str.split(%s: str, %sep: str, %maxsplit: i32) -> [str]

//Check if the current string starts with the specified prefix
func ir.str.startswith(%s: str, %prefix: str) -> bool

//Check if the current string starts with the specified suffix
func ir.str.endswith(%s: str, %substr: str) -> bool

//If the current string is all letters or numbers and contains at 
//least one character, return True; otherwise, return False
func ir.str.isalnum(%s: str, ) -> bool

//If the current string is all letters and contains at least 
//one character, return True; otherwise, return False
func ir.str.isalpha(%s: str, ) -> bool

//If the current string is all numbers and contains at least 
//one character, return True; otherwise, return False
func ir.str.isdigit(%s: str, ) -> bool

//If the current string is all lowercase letters and contains at least
//one eligible character, return True; otherwise, return False
func ir.str.islower(%s: str, ) -> bool

//If the current string is all uppercase letters and contains at least 
//one eligible character, return True; otherwise, return False
func ir.str.isupper(%s: str, ) -> bool

//If the current string is all blank characters and contains at least 
//one character that meets the criteria, return True; otherwise, return False
func ir.str.isspace(%s: str, ) -> bool

//Return a copy of the string that removes all corresponding characters 
//from the beginning and end of %s
//%chars - The set of characters that need to be removed
func ir.str.strip(%s:str, %chars: str, ) -> str

//Return a copy of the string that removes all corresponding characters 
//from the beginning of %s
//%chars - The set of characters that need to be removed
func ir.str.lstrip(%s:str, %chars: str, ) -> str

//Return a copy of the string that removes all corresponding characters 
//from the end of %s
//%chars - The set of characters that need to be removed
func ir.str.rstrip(%s:str, %chars: str, ) -> str

//Return a copy of the result after concatenating all strings 
//within %s using %seq as the delimiter
func ir.str.join(%s: str, %seq: [str], ) -> str

//Return a copy of the string, replacing all occurrences 
//of substrings %old with %new.
//%count - maximum number of replacements, if -1, there is no maximum limit
func ir.str.replace(%s: str, %old: str, %new: str, %count: i32, ) -> str

//Return a copy of the string after connecting %s with %sub
func ir.str.concat(%s: str, %sub: str, ) -> str


//Find the first occurrence of string %sub in %s
//%begin - the starting position of the search, the search range includes begin
//%end - the end position of the search, the search range does not include end
func ir.str.find(%s: str, sub: str, begin: i32, end: i32) -> u32

//Return a copy of the substring of [%begin,% end)
func ir.str.sub_str(%s: str, begin: i32, end: i32) -> u32

//Return the copy after inserting a string at the target position
func ir.str.insert(%s: %str, %sub: str, %index: i32,) -> str

//Convert strings to u8 array
func ir.str.to_bytes(%s: %str, ) -> [u8]

//Convert string to i128
func ir.str.to_i128(%s: %str, ) -> i128

//Convert string to u128
func ir.str.to_u128(%s: %str, ) -> u128
```


#### builtin
```solidity
//Terminate program
func ir.builtin.abort(%msg: str, )

func ir.builtin.require(cond: %bool,%msg: str, )

func ir.builtin.print(%msg: str, )

//Print type information for any value
func ir.builtin.print_type(%val: <ty>)

func ir.builtin.revert(%err_code: i32, %msg: str)

func ir.builtin.co_call(%co_name: str, %method_name: str, 
                        param1: <ty1>, param2: <ty2>,...) -> <ret_ty>

//Get the current block number
func ir.builtin.get_block_number() -> u64

//Get the current block timestamp
func ir.builtin.get_block_timestamp() -> u64

//Get the current block random seed
func ir.builtin.get_block_random_seed() -> str

//Get the sender of current transaction
func ir.builtin.tx_sender() -> str

//Get the hash of current transaction
func ir.builtin.tx_hash() -> str

//Get the index in the current block of current transaction
func ir.builtin.tx_index() -> u32

//Get the gas limit of current transaction
func ir.builtin.tx_gas_limit() -> u64

//Get the timestamp of current transaction
func ir.builtin.get_tx_timestamp() -> u64

//Get the nonce of current transaction
func ir.builtin.get_tx_nonce() -> u64

//Get the sender of current call
func ir.builtin.call_sender() -> str

//Get the contract address of current call
func ir.builtin.call_this_contract() -> str

//Get the caller contract address of current call
func ir.builtin.call_op_contract() -> str

//Get the gas limit of current call
func ir.builtin.call_gas_limit() -> u64

//Get the gas left of current call
func ir.builtin.call_gas_left() -> u64
```
### Metadata
IR metadata can be attached to type definitions, function definitions, and instructions:
```solidity
type <ty_name> = ... !<metadata_name> !<metadata_index> ...

//simple instruction
<instruction_name>(...) !<metadata_index> !<metadata_index> ...

//declaration instruction
let %<var_name> : <ty> !<metadata_index> !<metadata_index> ... = ...

//assignment instruction
%<var_name> : <ty> !<metadata_index> !<metadata_index> ... = ...


meta !<metadata_index> = { ... }

```

The main function of metadata is to attach additional information beyond IR, including high-level source code information (such as debug_info, native function annotations), as well as platform specific extension information.

Currently, IR has the following built-in metadata :

##### ir_debug_location
**Grammar**
```solidity
!ir_debug_location !<metadata_index>

!<metadata_index> = !{<start_line>: u64, <end_line>: u64, <file_name>: str, }
```
**Overview**
`ir_debug_location` is used to record the connection between IR nodes and the original high-level language source code
**Parameters**
`start_line`: Source statement start line number
`end_line`: Source statement end line number
`file_name`: Filepath of source file
**Examples**
```solidity
%0 = add(1, 2) !ir_debug_location !0
!0 = !{2: u64, 5: u64, "/Users/admin/ir/test.ir": str, }
```


##### extend_hostapi
**Grammar**
```solidity
!extend_hostapi !<metadata_index>

!<extend_hostapi> = !{<extend_name>: str, }
```
**Overview**
`extend_hostapi` are used to mark native functions declared in the source language, and this metadata will only have actual effects after being marked in the function declaration. If a function is marked as `extend_hostapi` then IR will skip the actual function body of the function in the codegen stage and instead generate a call instruction called the function marked by`extend_hostapi`.
**Parameters**
`extend_name`:  the function name of native function
**Examples**
```solidity
pub fn add_u64(%0: u64, %1: u64) -> u64 !extend_hostapi !0 {      
      
}

!0 = !{"native_quick_add_u64" :str, }
```
