# Rust-Rocket-LOGASS-API-Service
Rocket Rust microservice to provide Logging-As-A-Service using JSON API calls. This is analogous to the previous LOGASS application written in GoLang.

This project is meant to be a comparative to the same logging application written in golang, as well as getting to learn Rust.

Below are some demo runs for simple comparison, using defaults without optimization. Your results may vary.


## Demo of Various API Calls. ##
<img src="https://github.com/jiunnhwa/Rust-Rocket-LOGASS-API-Service/blob/main/20220715%20RustLogassDemo.gif" width=80% >



## Demo Insertion of 100 items using C# Client. ##
Average seems to range from 2-5 seconds. First round is always slowest, with subsequent rounds faster.  Source file is client.cs using atd lib System.Net.Http.

<img src="https://github.com/jiunnhwa/Rust-Rocket-LOGASS-API-Service/blob/main/20220715%20RustLogassClientInsertDemo.gif" width=80% >


## Demo Insertion of 100 items using Rust Client. ##
Sampling using rust client to insert items. Source file is client.rs using lib reqwest.
<img src="https://github.com/jiunnhwa/Rust-Rocket-LOGASS-API-Service/blob/main/20220716%20RustLogassClientInsertDemo.gif" width=80% >


## Demo Insertion of 100 items using Go Client. ##
Sampling using golang client to insert items. Source file is client.go using stdlib net/http.
<img src="https://github.com/jiunnhwa/Rust-Rocket-LOGASS-API-Service/blob/main/20220716%20RustLogassGoClientInsertDemo.gif" width=80% >


## Demo Insertion of 100 items using Python Client. ##
Sampling using Python client to insert items. Source file is client.py using lib aiohttp.
<img src="https://github.com/jiunnhwa/Rust-Rocket-LOGASS-API-Service/blob/main/20220716%20RustLogassPythonClientInsertDemo.gif" width=80% >


## Demo Insertion By All Clients. ##
Using Python, C#, Golang, Rust client to insert items at the same time. 
<img src="https://github.com/jiunnhwa/Rust-Rocket-LOGASS-API-Service/blob/main/20220716%20RustLogassAllClientsInsertDemo.gif" width=80% >

