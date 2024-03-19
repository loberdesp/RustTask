# Junior Rust Developer Task

Complete documentation for Real-time Currency Converter

## Features

- Currency Converter using real-time data with help of [ExchangeRate-API](https://www.exchangerate-api.com)
- User-friendly interface with interactive mode
- Error handling and explanation
- Listing available currencies
- Exchange rates caching to minimize API calls and improve performance

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

Project is written in rust and I assume you have certain C++ build tools installed on your machine

### Installing & Running

A step by step series of examples that tell you how to get a development env running

1. Download and install [rust](https://www.rust-lang.org/tools/install)

2. Clone repository - Run this command in cmd and enter folder

```
git clone https://github.com/loberdesp/RustTask.git
```

then

```
cd RustTask
```

3. Run build and execute command

```
cargo run
```

4. Congratulations! You should have running program!

### API

In case you need new API Key here are steps to acquire one and get it to work

1. Go to [ExchangeRate-API](https://www.exchangerate-api.com)

2. Click on big blue button in corner of the window: Get Free Key

3. Create account using your email and password

4. You should see Your API Key and example request on the screen

5. Grab API Key and open main.rs file located in cloned repository using text editor

6. There is one place where you enter your API Key, it is located in line 8 and looks like this

```
static API_KEY: &str = "MY_COOL_API_KEY";
```

7. All you have to do is swap MY_COOL_API_KEY with your new key you just got and you're good to go

### Optimization

Program is written in a way it minimizes API calls

When you call API to get exchange rates for one currency, it automatically saves all exchange rates for this currency and when you want to convert it to some other currency it doesn't call API and uses value stored in heap memory

Example:

1. You convert USD to PLN
2. USD is added to hashmap and it's exchange values are temporarily stored
3. You convert GBP to BIF
4. GBP is also added to hashmap
5. You want to convert USD to AWG, since USD was called and it's values are stored in memory program won't call API and instead it will get them from hashmap

### Docker

We will build docker image and run it, it supports running basic unit tests


You need [docker hub](https://hub.docker.com) installed on your machine, for testing in docker environment you have to use two commands:

1. Build docker image

```
docker build -t converter_test_img .
```

2. Run docker image for testing

```
docker run -t converter_test_img
```

3. Our image should pop up in docker and we should be able to control it with buttons