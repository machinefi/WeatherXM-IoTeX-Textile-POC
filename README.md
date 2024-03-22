# WeatherXM-IoTeX-Textile-POC

## POC Doc

https://textile.notion.site/IoTeX-WeatherXM-Textile-POC-Scope-1f2aaa580a134daea88f281b589f8b09

## Get started

1. Follow the README in https://github.com/machinefi/weatherxm-qod/blob/main/README.md#running-on-docker-locally to generate All WeatherXM QoD scores of all devices into `result.json` file.

Notes: a lightweight `result.json` example is provided in `./dataset` folder

2. Compile the `methods.rs` in `./zk` folder

Notes: a compiled `methods.rs` is provided in `./dataset` folder, or you can build it in `./zk` folder.

3. Create a w3bstream project 

**Via Sandbox Frontend** 

https://sandbox.w3bstream.com

- Upload `method.rs` as prover code
- Aggregation Amount: 1
- Proof Destination: Tableland
- Vault ID: `qod_poc_vault.data`


**Via ioctl**

 - Generate config for the project

```shell
// cli setup
ioctl config set wsEndpoint 'sprout-staging.w3bstream.com:9000'

ioctl ws project config -v "0.1" -t "risc0" -i "./dataset/methods.rs" -c "./dataset/textile.json"  -e "{\"image_id\":\"ZK_ID\", \"elf\":\"ZK_ELF\"}" -u "./dataset/textile_output_template.json"
```

 - Create a w3bstream project via `ioctl`


``` shell
ioctl ws project --contract-address 0x02feBE78F3A740b3e9a1CaFAA1b23a2ac0793D26 create --project-config-file ./dataset/textile.json 
```

4. Send raw data `result.json` via `ioctl` to w3bstream Server

``` shell
# Change the project id with yours
ioctl ws message send --project-id 87 --project-version "0.1" --data "{\"data\":\"$(cat ./dataset/result.json)\", \"receipt_type\":\"Stark\"}" 
```

5. Query the result 

 - The status of message on the  w3bstream Server could be checked via `ioctl`

``` shell
# Change the message id with yours
ioctl ws message query -i c4a2be1c-def3-463f-813a-12efc7e46856 
```

 - Check the storage result via [Basin-Cli](https://github.com/tablelandnetwork/basin-cli/tree/main?tab=readme-ov-file#listing-events) with the vaultID `qod_poc_vault.data`