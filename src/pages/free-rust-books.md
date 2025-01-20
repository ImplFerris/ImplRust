---
layout: ../layouts/MarkdownLayout.astro
title: Index of all online Rust books
---

# Index of Free Online Rust Books
A curated list of free online books about Rust, organized by category.

## Starter Books for Beginners
- **[The Rust Programming Language](https://doc.rust-lang.org/book/)** : Start your Rust journey with the book.
- **[Free Rust course developed by the Android team at Google](https://google.github.io/comprehensive-rust/)**: The course is used internally at Google when teaching Rust to experienced software engineers. They typically have a background in C++ or Java. The course covers all aspects of Rust, from basic syntax to generics and error handling. It also includes deep dives on Android, Chromium, bare-metal, and concurrency.
- **[Yet Another Rust Resource (YARR!)](https://yet-another-rust-resource.pages.dev/introduction)** : Software engineers who know how to program in a high-level language but aren't familiar with lower-level programming and want to learn to write Rust quickly
- **[The Rust Book (Abridged)](https://jasonwalton.ca/rust-book-abridged/)** : condensed version of "The Rust Programming Language". If you're already familiar with one or more other programming languages, then you are likely already familiar with a lot of the concepts the book covers, and you might benefit from this shorter version.
- **[Rust 101](https://rust-lang.guide/)** : A guide to aid you in your journey of becoming a Rustacean (Rust developer).
- **[Rust for C#/.NET Developers](https://microsoft.github.io/rust-for-dotnet-devs/latest/)** : a (non-comprehensive) guide for C# and .NET developers that are completely new to the Rust.

## Exercises To Learn Rust

- **[100 Exercises To Learn Rust](https://rust-exercises.com/100-exercises/)** : This course will teach you Rust's core concepts, one exercise at a time. You'll learn about Rust's syntax, its type system, its standard library, and its ecosystem.
- **[Rust By Example](https://doc.rust-lang.org/rust-by-example/)** : Learn Rust with examples (Live code editor included)
- **[Rust By Practice](https://practice.course.rs/)** : This book was designed for easily diving into and getting skilled with Rust, and it's very easy to use: All you need to do is to make each exercise compile without ERRORS and Panics !

## Concurrency
- **[Rust Atomics and Locks](https://marabos.nl/atomics/foreword.html)** : Provides an excellent overview of low-level concurrency
- **[async-book](https://rust-lang.github.io/async-book/index.html)** : Asynchronous Programming in Rust
- **[Async programming in Rust with async-std](https://book.async.rs/)** : This book serves as high-level documentation for async-std and a way of learning async programming in Rust through it. As such, it focuses on the async-std API and the task model it gives you.

## CLI
- **[Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)** : Documentation on how to use the Rust Programming Language to develop commandline applications
- **[PNGme: An Intermediate Rust Project](https://jrdngr.github.io/pngme_book/introduction.html)** : Make a command line program that lets you hide secret messages in PNG files

## Macros
- **[The Little Book of Rust Macros](https://veykril.github.io/tlborm)** : This book is an attempt to distill the Rust community's collective knowledge of Rust macros, the Macros by Example ones as well as procedural macros(WIP).
- **[MacroKata](https://tfpk.github.io/macrokata/)** : a set of exercises which you can use to learn how to write macros in Rust. When completing each task

## FFI
- **[Safe interop between Rust and C++](https://cxx.rs/)** :  CXX is a library provides a safe mechanism for calling C++ code from Rust and Rust code from C++.
- **[The (unofficial) Rust FFI Guide](https://www.michaelfbryan.com/rust-ffi-guide/)** :  Doing more in-depth FFI tasks than simply calling one or two functions from a C library.  This guide is centred around the idea of building a REST client using Qt (C++) for the GUI and reqwest (Rust) for the business logic.

## Embedded
- **[The Embedded Rust Book](https://docs.rust-embedded.org/book/)** :  An introductory book about using the Rust Programming Language on "Bare Metal" embedded systems, such as Microcontrollers.
- **[impl Rust on ESP32 Book](https://esp32.implrust.com/)**: A hands on book on embedded programming with ESP32 and explore various sensors and peripherals
- **[Pico Pico Book](https://pico.implrust.com/)**: A hands on book on embedded programming with Pico2(RP2350) and explore various sensors and peripherals
- **[Discovery - Microbit](https://docs.rust-embedded.org/discovery/microbit/)** :  This book is an introductory course on microcontroller-based embedded systems that uses Rust as the teaching language rather than the usual C/C++.  Uses Microbit board (v1 and v2)
- **[Discovery - STM32F3DISCOVERY](https://docs.rust-embedded.org/discovery/f3discovery/)** :  This book is an introductory course on microcontroller-based embedded systems that uses Rust as the teaching language rather than the usual C/C++. Uses STM32F3DISCOVERY board.
- **[The Rust on ESP Book](https://docs.esp-rs.org/book/)** :  This book aims to provide a comprehensive guide on using the Rust programming language with Espressif SoCs and modules.
- **[Embedded Rust on Espressif](https://docs.esp-rs.org/std-training/)** :  This repository contains Training Material for learning to use Embedded Rust with the Espressif ESP32-C3.
- **[Embedded Rust (no_std) on Espressif](https://docs.esp-rs.org/no_std-training/)** :  Getting-started guide on using the Rust with Espressif SoCs using no_std.
- **[Workbook for Embedded Workshops](https://embedded-trainings.ferrous-systems.com/)** :  It uses both the nRF52840 Development Kit (DK) and the nRF52840 Dongle. It mainly develop programs for the DK and use the Dongle to assist with some exercises.

## Kernel Development
- **[OS Development on the Raspberry Pi](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)** :  This is a tutorial series for hobby OS developers who are new to ARM's 64 bit ARMv8-A architecture. 
- **[Writing an OS in Rust](https://os.phil-opp.com/)** : Blog series creates a small operating system in the Rust

## Linux Development
- **[Building eBPF Programs with Aya](https://aya-rs.dev/book/)** : This getting started guide will help you use the Rust Programming Language and Aya library to build extended Berkley Packet Filter (eBPF) programs.

## WASM
- **[The Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)** : This book is for anyone interested in compiling Rust to WebAssembly for fast, reliable code on the Web
- **[Chip-8 Book](https://github.com/aquova/chip8-book)**: Introductory tutorial for how to develop your first Chip-8 emulator, targeting both desktop computers and web browsers via WebAssembly.

## Compiler Development
- **[Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/)** : A guide to how rustc works and how to contribute to it.
- **[Writing Interpreters in Rust: a Guide](https://rust-hosted-langs.github.io/book/introduction.html)** : The book will walk through the basics of interpreted language implementation in Rust with a focus on the challenges that are specific to using Rust.
- **[Build a Lua Interpreter in Rust](https://wubingzheng.github.io/build-lua-in-rust/en/)** : This series of articles introduces the implementation of a Lua interpreter from scratch in the Rust language.
- **[Create your own programming language with Rust](https://createlang.rs/01_calculator/repl.html)**

## Network Programming
- **[Building a DNS server in Rust](https://github.com/EmilHernvall/dnsguide)**: A guide to writing a DNS Server from scratch in Rust


## Unsafe Rust
- **[The Rustonomicon](https://doc.rust-lang.org/nomicon/)**: The book digs into all the awful details that you need to understand when writing Unsafe Rust programs.
- **[Learn Rust the Dangerous Way](https://cliffle.com/p/dangerust/)** : A series of articles putting Rust features in context for low-level C programmers who maybe don’t have a formal CS background — the sort of people who work on firmware, game engines, OS kernels, and the like.


## DSA and Design Patterns
- **[DSA Book](https://github.com/QMHTMY/RustBook)** : A book about Rust Data Structures and Algorithms.
- **[Rust Design Patterns](https://rust-unofficial.github.io/patterns/)** :  A catalogue of Rust design patterns, anti-patterns and idioms

## Machine Learning
- **[Rust Machine Learning Book](https://rust-ml.github.io/book/)** :  The aim of this book is to demonstrate how the Rust language can be used for Machine Learning tasks.

## General
- **[Effective Rust](https://www.lurklurk.org/effective-rust/)** : 35 Specific Ways to Improve Your Rust Code. This practical guide helps you make the transition to writing idiomatic Rust—while also making full use of Rust's type system, safety guarantees, and burgeoning ecosystem.
- **[Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html)** : Teach you basic and advanced Rust programming entirely by having you implement 6 linked lists
- **[Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)** : This Rust Cookbook is a collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the Rust ecosystem.
- **[High Assurance Rust: Developing Secure and Robust Software](https://highassurance.rs/)** :  This book teaches you how to build reliable and secure software using Rust through hands-on projects.
- **[The Rust Unstable Book](https://doc.rust-lang.org/stable/unstable-book/)** 
- **[Rust Fuzz Book](https://rust-fuzz.github.io/book/)**: Fuzz testing is a software testing technique used to find security and stability issues by providing pseudo-random data as input to the software.

