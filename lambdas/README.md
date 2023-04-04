# lambdas

Repository containing all the lambda endpoints for API Gateway.

## Requirements

- [Cargo Lambda]

## Testing

2 terminal are required to test the functions.

### Terminal 1

The first terminal is used to watch the functions, meaning it will emulate the
AWS Lambda control plane API.

Launch it with the following command:

```bash
cargo lambda watch
```

The verbose flag `-v/-vv` can be added to increase the log level, and therefore
the amount of information being output.

### Terminal 2

The second terminal will serve to send requests to the Lambda emulator.

Since we are emulating sending the requests through API Gateway, the payloads
must simulate what API GW would send to the Lambda function.

Such payloads are stored in the `/src/fixtures` folder, and invoking request
follows the following pattern:

```bash
cargo lambda invoke ${LAMBDA} --data-file lambdas/src/fixtures/${LAMBDA}.json
```

For instance:

```bash
cargo lambda invoke get-cities --data-file lambdas/src/fixtures/get-cities.json
```

[cargo lambda]: https://www.cargo-lambda.info/
