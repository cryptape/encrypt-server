# sm-server
Sm-server is an HTTP server that provides sm algorithm services

# Quick start
```shell
$ git clone git@github.com:cryptape/sm-server.git
$ PORT=8888 cargo run
$ curl http://localhost:8888/ping
```

# API

## Overview
| Name | Method | Path |
| ------ | ------ | ------ |
| Create keypair  | POST | /sm2/keypair |
| Sign message | POST | /sm2/raw/signature |
| Sign digest | POST | /sm2/digest/signature |
| Verify message | POST | /sm2/raw/verification |
| Verify digest | POST | /sm2/digest/verification |

## Details
### Create keypair
`POST /sm2/keypair`

Response:
```json
201 created
{
  "privateKey":"0x7b27257ceda1887f1e1aaf04413d58815f492ed532dd4f5af453e05341a020f",
  "publicKey":"0x04cfec2eacc1bceef14979384979277221f666638194b644699a0dd37dc6b6fa42a59eb36b3d7cee2dcd68b5532f79c5048f8c7ee1d5824886e552f80220763f9"
}
```

### Sign message
`POST /sm2/raw/signature`

Request:

| Name | Type | example |
| ------ | ------ | ------ |
| body.privateKey  | Hex | 0xb121c57731f8ccad61192a4da2fef5fa8c4f500d9a7a0c24a07e6d1eb9fc9c1c |
| body.raw| Hex | 0xfffff |

Response:
```json
201 created
{
	"signature":"0xe468305c4b5779c112bbc2d9215b630b845e89aca4b30790a857db136c50176856717e15f083b3dd00280fc7ccc971acc5cbf841bf43d2c1a1d223d88a24f41a3079049f2d9bbd60b4790b8a9e0b08d395b9b8e2699a858520b78c6eff90ffd3222aeb9d72ebb5ab1bd4a59c0d8b06a2c71c7fcdbbfd350eaed68bfd0dd48db4"
}
```

### Sign digest
`POST /sm2/digest/signature`

Request:

| Name | Type | example |
| ------ | ------ | ------ |
| body.privateKey  | Hex | 0xb121c57731f8ccad61192a4da2fef5fa8c4f500d9a7a0c24a07e6d1eb9fc9c1c |
| body.digest| Hash | 0xf7dd52f4014e2852d8d9d9d2914fc87bc3eb9fc4d92434dda73509e416103de3 |

Response:
```json
201 created
{
	"signature":"0xe468305c4b5779c112bbc2d9215b630b845e89aca4b30790a857db136c50176856717e15f083b3dd00280fc7ccc971acc5cbf841bf43d2c1a1d223d88a24f41a3079049f2d9bbd60b4790b8a9e0b08d395b9b8e2699a858520b78c6eff90ffd3222aeb9d72ebb5ab1bd4a59c0d8b06a2c71c7fcdbbfd350eaed68bfd0dd48db4"
}
```

### Verify message
`POST /sm2/raw/verification`

Request:

| Name | Type | example |
| ------ | ------ | ------ |
| body.publicKey  | Hex | 0x042aa6a8773c2518c475dfa866a30d22293c10c5a4980740d87157642fada33b8da916fc1c1e87cb6bd2f7184679159970a26eed9756e62d9a040cf9dffd2b7e7f |
| body.raw| Hex | 0xffff |
| body.signature| Hex | 0xda54f39a22fa05abf2948c252a44d5e90d26db7f435d5215877cb1c6c84e84fea5e5c9e84b220d7035c344a85690c35200ac5a4fa68e6aa4eda9fae12bfbc3312aa6a8773c2518c475dfa866a30d22293c10c5a4980740d87157642fada33b8da916fc1c1e87cb6bd2f7184679159970a26eed9756e62d9a040cf9dffd2b7e7f |

Response:
```json
200 ok
{
  "result": true
}
```

### Verify digest
`POST /sm2/digest/verification`

Request:

| Name | Type | example |
| ------ | ------ | ------ |
| body.publicKey  | Hex | 0x042aa6a8773c2518c475dfa866a30d22293c10c5a4980740d87157642fada33b8da916fc1c1e87cb6bd2f7184679159970a26eed9756e62d9a040cf9dffd2b7e7f |
| body.digest| Hash | 0xf7dd52f4014e2852d8d9d9d2914fc87bc3eb9fc4d92434dda73509e416103de3 |
| body.signature| Hex | 0xda54f39a22fa05abf2948c252a44d5e90d26db7f435d5215877cb1c6c84e84fea5e5c9e84b220d7035c344a85690c35200ac5a4fa68e6aa4eda9fae12bfbc3312aa6a8773c2518c475dfa866a30d22293c10c5a4980740d87157642fada33b8da916fc1c1e87cb6bd2f7184679159970a26eed9756e62d9a040cf9dffd2b7e7f |

Response:
```json
200 ok
{
  "result": true
}
```

