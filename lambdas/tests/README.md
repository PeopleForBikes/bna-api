# Integration tests

The integration tests use [Hurl] and [Schemathesis].

## Preparing the assets

The assets created for the tests are prepared using the 2024 historical data and
the [xsv] tool.

```bash
xsv search -i -s country 'United States' city-ratings-all-historical-results-v24.07.csv | xsv sample 10 > sample-us.csv
xsv search -i -v -s country 'United States' city-ratings-all-historical-results-v24.07.csv | xsv sample 10 > sample-non-us.csv
xsv cat rows sample-us.csv sample-non-us.csv > sample-all-25.01.csv
```

##

[Hurl]: https://hurl.dev
[Schemathesis]: https://github.com/schemathesis/schemathesis
[xsv]: https://github.com/BurntSushi/xsv
