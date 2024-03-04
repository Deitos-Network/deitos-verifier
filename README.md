# Deitos File verifier

This service is one component of the Deitos Data Integrity Protocol.


## Configuration

The configuration is done through the following environment variables:
- `DV_PORT`: The port to listen on. Defaults to `3030`.
- `HDFS_URI`: The URI of the WebHDFS server to connect to. Defaults to `http://localhost:50070/webhdfs/v1/data/deitos`.

## Installation

```
cargo build --release
```

## Usage

### Run 

```
DV_PORT="<PORT>" HDFS_URI="<HDFS URI>"  target/release/deitos-verifier
```

### Query 


```
curl "<VERIFIER_URL>/<FILE_NAME>"
```

**Example**:

```
curl "http://localhost:4040/test.csv"
```