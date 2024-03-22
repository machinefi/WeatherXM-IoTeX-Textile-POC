# WeatherXM-IoTeX-Textile-POC

## POC Doc

https://textile.notion.site/IoTeX-WeatherXM-Textile-POC-Scope-1f2aaa580a134daea88f281b589f8b09

## Get started

1. Follow the README in https://github.com/machinefi/weatherxm-qod/blob/main/README.md#running-on-docker-locally to generate All WeatherXM QoD scores of all devices into `result.json` file.

notes: a lightweight `result.json` example is provided in `./dataset` folder

2. Compile the `methods.rs` in `./zk` folder

notes: a compiled `methods.rs` has been provided in `./dataset` folder, or you can build it in `./zk` folder.


3. Generate config for the project

```shell
// cli setup
ioctl config set wsEndpoint 'sprout-staging.w3bstream.com:9000'

ioctl ws project config -v "0.1" -t "risc0" -i "./dataset/methods.rs" -c "./dataset/textile.json"  -e "{\"image_id\":\"ZK_ID\", \"elf\":\"ZK_ELF\"}" -u "./dataset/textile_output_template.json"
```

4. Create a w3bstream project via `ioctl`


``` shell
ioctl ws project --contract-address 0x02feBE78F3A740b3e9a1CaFAA1b23a2ac0793D26 create --project-config-file ./dataset/textile.json 
```

5. Send `result.json` via `ioctl` to w3bstream Server


``` shell
ioctl ws message send --project-id 82 --project-version "0.1" --data "{\"data\":\"$(cat ./dataset/result.json)\", \"receipt_type\":\"Stark\"}" 
```

6. Check the result via [Basin-Cli](https://github.com/tablelandnetwork/basin-cli/tree/main?tab=readme-ov-file#listing-events) with the vaultID `qod_poc_vault.data`