# The Smart Intermediate Representation

[![Release][release-shield]][release-url] [![Issues][issues-shield]][issues-url] [![Apache License][license-shield]][license-url]

The Smart Intermediate Representation(short for IR) project is a new compiler framework intended for smart contract development, which is mainly inspired by LLVM, MLIR and Solidity/Yul. The compiler's goals are:

* Trust and Security
* Rollup Friendly
* AI Friendly
* High Performance and Low Gas Cost

## Table of Contents

- [The Smart Intermediate Representation](#the-smart-intermediate-representation)
  - [Table of Contents](#table-of-contents)
  - [Background](#background)
  - [Installation](#installation)
  - [Quick Start](#quick-start)
    - [Example](#example)
    - [Build the Docker Image for the Command Line Tool](#build-the-docker-image-for-the-command-line-tool)
    - [Build a Test IR Program](#build-a-test-ir-program)
  - [Documentation](#documentation)
  - [Roadmap](#roadmap)
    - [V0.1](#v01)
  - [Contribution](#contribution)
  - [Community](#community)
  - [License](#license)

## Background

Since the circulation of Blockchain and Smart Contracts, there have been many incidents of exploitation. These incidents are mainly caused by attacks on unsafe smart contracts such as reentrancy issues, and compiler bugs. In the existing cases, it is difficult to investigate or locate once the attack occurs. Most safe/secure cases also require 3rd-party auditing which is very costly.

Meanwhile, to wring the most performance out of a regular gas allowance, or something else. It's important to optimize both on-chain and off-chain scenarios.

Currently, we propose a universal technical solution to be adapted to any programming language to help blockchain platform/developer make their smart contract safer, high performance and multi-scenario compatibility. 

## Installation

Instructions about how to build and install the Smart Intermediate Representation compiler can be found in the [Development Guide](./doc/Development_Guide.md)

## Quick Start

### Example

Here is a "Hello World" hand-written program with debug information in the IR:

```sir
module_name = "HelloWorld"
contract HelloWorld {
    state {
    }
    pub fn HelloWorld.HelloWorld.init()  {
        0:
            ret()
    }

    pub fn HelloWorld.HelloWorld.greeting() -> str {
        1:
            call(@ir.builtin.print("hello": str, ) -> void, )
            ret("Hello world!": str, )
    }

}
```

For more examples of IR textual format programs, you can see [IR Example](./ir_example)

### Fetch the Docker Image and Build a Test IR Program

```shell
docker run smartir/cli:latest ir_cli build sir/example/hello_world.ir
```

## Documentation

The IR Specification document is [here](./doc/specification/SmartIR.md)

## Roadmap

> **📢 NOTE: This roadmap is tentative, it will be adjusted at any time**

The IR has a high level of compatibility with many blockchains. Here is a brief description of what we envision for the next versions.

### V0.1

| Feature                                              | Status      | Show Case                |
|------------------------------------------------------|-------------|--------------------------|
| Support Solidity/Yul for Ethereum-Compatible         | In progress | [Yul to IR](./yul_to_ir)    |
| Support IR-AI-based Intent Consistent Analysis       | In progress | [AI Intent](./ai_intent) |
| Provide a default high-level frontend language       | In progress | -                        |
| Enhance Runtime Performance and Lowering Gas Cost    | In progress | -                        |
| Support Formal Verification via Static Prover        | In progress | -                        |
| Support ZKP by Abstract Circuit IR                   | Not started | -                        |
| Provide User-Defined Frontend guide                  | Not started | -                        |
| Provide Solidity -> WASM-based Layer 2 porting guide | Not started | -                        |
| Provide a default optimized VM based on wasmtime     | Not started | -                        |

## Contribution

The IR is still under development. Contributions are always welcome. Please follow the [Contributing Guide](./doc/Contributing%20Guide.md) if you want to make a new feature for IR.

## Community

![wechat group](./assets/wx.JPG)

![dingtalk group](./assets/dingtalk.png)

<!--
* Join us on the [Discord]()
-->

## License

[Apache 2.0](./LICENSE)

[release-shield]: https://img.shields.io/github/actions/workflow/status/AntChainOpenLabs/Smart-Intermediate-Representation/release.yml.svg?style=for-the-badge&label=Release
[release-url]: https://github.com/AntChainOpenLabs/Smart-Intermediate-Representation/releases/tag/v0.0.1-alpha.2
[license-shield]: https://img.shields.io/badge/License-Apache_2.0-green.svg?style=for-the-badge
[license-url]: ./LICENSE
[issues-shield]: https://img.shields.io/github/issues/AntChainOpenLabs/Smart-Intermediate-Representation.svg?style=for-the-badge
[issues-url]: https://github.com/AntChainOpenLabs/Smart-Intermediate-Representation/issues
