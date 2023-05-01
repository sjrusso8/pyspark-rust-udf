# PySpark Rust UDF

This is a simple example of how to use Rust UDFs in PySpark. It uses the [PyO3](https://pyo3.rs/v0.18.3/) crate to create a Python module that can be imported into PySpark.

The functions are based on the benchmark notebook from the original Databricks blog post for [Introducing Pandas UDF for PySpark](https://www.databricks.com/blog/2017/10/30/introducing-vectorized-udfs-for-pyspark.html). For the original implementation of the benchmark, check the [Pandas UDF Notebook](https://databricks-prod-cloudfront.cloud.databricks.com/public/4027ec902e239c93eaaa8714f173bcfc/1281142885375883/2174302049319883/7729323681064935/latest.html).

## Requirements

- Rust 1.68.0
- maturin 0.14.17

## Build

To build the Rust module, run the following command:

```bash
python -m venv .env
source .env/bin/activate
maturin develop
```

The `maturin` tool helps build and deploy rust packages built with the pyo3 crate.

## Benchmark Comparison

The benchmark was done on a single node cluster running Databricks Runtime 11.3. Overall, the Rust UDFs can be faster than vanilla Python UDFs, but are slower than the Pandas UDFs.

To run the benchmark, import the notebook under `notebooks/` into your Databricks workspace. You will need to create a cluster and use the web terminal on the driver to install rust and the pip packages. You can then run the notebook.

A spark dataframe was created with 10000000 records, and two columns. ID and a random double value using the pyspark functions `rand()`.

The benchmark results are below:

- plus_one
  - python: 3.48 s ± 141 ms
  - pandas: 1.77 s ± 409 ms
  - rust: 3.9 s ± 61.8 ms
- cumulative_distribution_function
  - python: 188s ± 890 ms
  - pandas: 1.55 s ± 72 ms
  - rust: 4.11 s ± 112 ms
- subtract_mean (groupby)
  - python: 40.9 s ± 1.78 s
  - pandas: 2.82 s ± 124 ms
  - rust: 3.44 s ± 97.5 ms

## Conclusion

Would you ever use Rust UDFs in PySpark? Probably not, but it's a fun experiment. I would recommend trying to avoid UDFs in general, but if you have to use them, Pandas UDFs are the way to go.

If you think there's a better way to implement the Rust UDFs, please open an issue or PR. I did not want to compare UDF created with other rust backed libraries like Polars, DataFusion, or Apache Arrow, but I would be interested in seeing how they compare.

## Blog Post

I wrote a blog post about this project, which you can find [here]()