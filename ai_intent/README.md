> **📢 NOTE: This feature is in a very early PoC stage, the complete version will be released before 2024 Q1**

# AI Intent

## Introduction

The problem of inconsistency between code implementation logic and the author's intention is widespread. This phenomenon is known as Code-Comment Inconsistency (CCI). Especially in the field of smart contracts, CCI can lead to serious vulnerabilities and economic losses. Smart Intermediate Representation (hereinafter referred to as SIR) is an IR dedicated to smart contract scenarios. Intent consistency detection is one of the target problems that SIR wants to solve.

Intent consistency analysis based on SIR can be regarded as a user-friendly artificial intelligence Lint tool provided for users. We obtain a large amount of public smart contract code from Etherscan, convert the corresponding SIR into an AI model through feature extraction, and extract function-level comments as intentions to train the AI model. Compared with some existing contract audit technologies and solutions, such as expert audit, formal verification-theorem proof (Coq), and symbolic execution (SMT Solver), SIR-based intent consistency analysis provides a more lightweight solution. For developers who lack expert experience in smart contract auditing, SIRs can be mapped to AI models and the consistency of business intentions and code logic can be verified through the intent consistency tool. In addition to the logic of the code itself, we also pay attention to the rich and complex business intentions of some natural languages in the blockchain and Web3 scenario, such as "burn" corresponding to the reduction of balance in smart contracts. Therefore, compared with other CCI detection tools based on general programming languages, SIR-based intent consistency analysis can better handle CCI issues in blockchain and Web3 scenarios.

## Features

### Intent consistency detection

One of the core features of this project is the use of SIR and AI technology to automatically detect inconsistencies between comments and implementation logic in the code. Through SIR, analysis tools can deeply analyze the structure and semantics of source code. This high-level abstraction enables AI models to more accurately understand the intent of the code and identify portions of logic that do not match comments. The analysis reports provided by the intent consistency analysis tool help developers quickly locate and fix these potential problems, thereby ensuring contract code quality and improving maintainability. In this way, our tool greatly simplifies the process of code auditing.

### Fork code consistency detection

Different from common languages, another scenario of intention consistency in the smart contract field is to detect the consistency between Fork code and source code. For developers, there may be a scenario where they fork a piece of code and make some minor changes. For example, fork an ERC20 standard token contract, make some changes and then release a new code contract. The question is, do these changes change the intent of the source code? In addition to the consistency of the code with the intent described in the natural language in the comments, we also hope to support the detection of consistency between the two contracts. When we perform this type of analysis based on IR, we can gain deeper insight into the underlying intent of the code without being disturbed by changes in superficial features such as variable naming. IR provides a more abstract and standardized view of the code by extracting the structural elements of the source code and stripping away specific language syntax and lexical details (such as variable names and syntactic sugar). This abstraction allows analysis to focus on program logic and behavior, allowing more accurate inferences about program intent and the impact of code changes.

## Technical Architecture

![tech](/ai_intent/tech.jpg)

The technical architecture mainly includes the following parts:
First, we collect a large number of open-source smart contract codes from Etherscan, which provides a rich sample set to support the training of our model. We then use compilation technology to extract function-level code comments in smart contracts, while converting source code written in various front-end languages (such as Solidity) into a SIR. SIR not only retains the complete logical information of the source code but also eliminates unnecessary details, providing a clear and abstract data representation for subsequent analysis.
Subsequently, we perform in-depth feature extraction on SIR and convert it into a heterogeneous graph model. This step takes advantage of the structural characteristics of SIR and provides the necessary input for the AI model. Through the heterogeneous graph model, we can capture the essential characteristics of the code, including control flow and data dependencies, which are the key to understanding the semantics of the code.
Armed with a structured heterogeneous graph model, we can train our AI model to identify and verify consistency between comments and code implementations. The model is trained on a large amount of smart contract code and learns how to identify reasonable code behavior that matches annotations.
In actual applications, when the user inputs a piece of code to be tested, our tool will use the same method as the training phase to extract the comments of the test code and generate SIR, and then use the trained AI model to verify the consistency of comments and code logic. If inconsistencies are found, the tool will provide feedback and pinpoint potential problem areas.
Through these methods, we provide an efficient, low-cost, user-friendly intent consistency detection tool for smart contract auditing. This tool not only improves the automation of the audit process and reduces the complexity and time required of traditional manual audits, but also reduces potential human errors through intelligent analysis methods. Users can simply and quickly submit code and let the system automatically perform consistency checks on comments and code logic, without the need for special training or complicated setup processes.

## Demo

Below is an ERC20 contract represented by SIR. We "mistakenly" swapped the operators, causing the operation logic to be inconsistent with the intention in the comments.

![SIR](/ai_intent/SIR.png)

After checking with the intent consistency tool, we can get a consistency score and consistency determination results. Such results can help us check the code that may have problems.

![result](/ai_intent/result.png)

## Future Plan

Currently, our tools mainly focus on function-level intent detection when detecting intent consistency issues. With the development of the project and the advancement of technology, we plan to further refine the model training so that the detection capability can be extended to a more detailed level, such as to the level of a single statement represented by SIR.
By training and optimizing models more deeply, future tools will be able to reveal inconsistencies between code and annotations at a more precise level, providing developers with more specific and detailed feedback. This will not only improve detection accuracy but also make problem location faster and more intuitive, thus greatly improving the efficiency of contract correction.
Additionally, future reports will not be limited to identifying possible locations of inconsistencies but will include more comprehensive analysis results such as possible causes of inconsistencies, business logic impact assessments, and even possible fix recommendations. This will provide developers with a more comprehensive view, helping them better understand the behavior of their code and how to improve it to ensure the accuracy and completeness of comments.
