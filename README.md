# SugarFunge API

#### Copy the environment file as .env and make the changes based on your needs

```
cp .env.example .env
```

## Launch API server
```
cargo run
```



## Help
```
sugarfunge-api 0.1.0

USAGE:
    sugarfunge-api [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --db-uri <db>                  
    -l, --listen <listen>               [default: http://127.0.0.1:4000]
    -s, --node-server <node-server>     [default: ws://127.0.0.1:9944]
```

## Generate SugarFunge Types
```
subxt-cli metadata -f bytes > sugarfunge_metadata.scale
```

## Environment configuration
- Example environment file: **.env.example**

| Variable Name               | Description                             |
| --------------------------- | --------------------------------------- |
| KEYCLOAK_HOST               | Keycloak base URL                       |
| KEYCLOAK_REALM              | Keycloak realm used                     |
| KEYCLOAK_CLIENT_ID          | Keycloak client used                    |
| KEYCLOAK_CLIENT_SECRET      | Keycloak client secret                  |
| KEYCLOAK_USERNAME           | Keycloak username                       |
| KEYCLOAK_USER_PASSWORD      | Keycloak user password                  |