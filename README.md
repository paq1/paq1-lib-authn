# paq1-lib-authn

## run unit test
```cmd
 cargo test --test run_unit_tests
```

## run integration test
il est necessaire d'avoir une mongo local qui tourne, conformément aux config des TI
```cmd
 cargo test --test run_integration_tests
```


## run unit test in infra
```cmd
 cargo test -p infra --test run_unit_tests
```

## run integration test in infra
```cmd
 cargo test -p infra --test run_integration_tests
```

## run all with coverage and report

il est necessaire d'avoir une mongo local qui tourne, conformément aux config des TI

```cmd
cargo tarpaulin --all-features --workspace --all-targets --out Html
```