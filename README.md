# mdp

This project is a Rust-based tool for downloading historical financial data from the Marketstack API.

## Features

- Downloads historical data, including closing prices, opening prices, volumes, and more.
- Configurable with start and end dates, result limits, and instrument symbols.
- Handles HTTP errors and parameter validation.
- Supports default values for optional parameters.

## Prerequisites

Make sure you have the following tools installed:

- Rust (version 1.65 or higher recommended)
- An account on Marketstack to obtain a valid API Key.

## Project Setup

1. Clone this repository:

## Input

- **api_key**:
- **`symbols`**: A list of stock symbols for which the data will be downloaded.
- **`date_from`**: The start date for the data range in `YYYY-MM-DD` format.
- **`date_to`**: The end date for the data range in `YYYY-MM-DD` format.
- **`limit`**: Specifies the maximum number of data entries retrieved per API request.
  - **Default**: 1000
  - **Maximum**: 1000
  - **Minimum**: 100
