# WeatherXM-IoTeX-Textile-POC

## POC Doc

https://textile.notion.site/IoTeX-WeatherXM-Textile-POC-Scope-1f2aaa580a134daea88f281b589f8b09

## Get started

1. Follow the README in https://github.com/machinefi/weatherxm-qod/blob/main/README.md#running-on-docker-locally to generate All WeatherXM QoD scores of all devices into `result.json` file.

notes: a lightweight `result.json` is provided in `./dataset` folder

2. Create a w3bstream project on https://sandbox.w3bstream.com

notes: compiled `methods.rs` is provided in `./dataset` folder

3. Send `result.json` via `ioctl` to w3bstream Server

```shell
ioctl config set wsEndpoint 'sprout-staging.w3bstream.com:9000'

// TODO: fix this
ioctl ws message send --project-id 43 --project-version "0.1" --data "{\"private_input\":\"20.5\", \"public_input\":\"12.8\", \"receipt_type\":\"Snark\"}" 
```

4. Check the result via [Basin-Cli](https://github.com/tablelandnetwork/basin-cli/tree/main?tab=readme-ov-file#listing-events)