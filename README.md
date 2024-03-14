# WeatherXM-IoTeX-Textile-POC

## POC Doc

https://textile.notion.site/IoTeX-WeatherXM-Textile-POC-Scope-1f2aaa580a134daea88f281b589f8b09

## Get started

1. Follow the README in https://github.com/machinefi/weatherxm-qod/blob/main/README.md#running-on-docker-locally to generate All WeatherXM QoD scores of all devices into `result.json` file.

notes: a lightweight `result.json` example is provided in `./dataset` folder

2. Compile the `methods.rs` in `./zk` folder

notes: a compiled `methods.rs` has been provided in `./dataset` folder, or you can build it in `./zk` folder.

3. Create a w3bstream project on https://sandbox.w3bstream.com

4. Send `result.json` via `ioctl` to w3bstream Server

```shell
ioctl config set wsEndpoint 'sprout-staging.w3bstream.com:9000'

ioctl ws code convert -v "0.1" -t "risc0" -i "./dataset/methods.rs" -c "./config/textile.json"  -e "{\"image_id\":\"ZK_ID\", \"elf\":\"ZK_ELF\"}"

``` 
add `output` in the `./config/textile.json`, like `./config/textile.json.example`
``` json
      "output": {
        "type": "textile",
        "textile": {
          "vaultID": "qod_poc.data"
        }
      }
```

``` shell
// the command will get a project id
ioctl ws project --contract-address xxx create --project-config-file ./config/textile.json 

ioctl ws message send --project-id 43 --project-version "0.1" --data "{\"data\":\"$(cat ./dataset/result.json)\", \"receipt_type\":\"Stark\"}" 
```

5. Check the result via [Basin-Cli](https://github.com/tablelandnetwork/basin-cli/tree/main?tab=readme-ov-file#listing-events) with the vaultID `qod_poc.data`