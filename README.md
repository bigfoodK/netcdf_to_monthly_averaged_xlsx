# netcdf_to_monthly_averaged_xlsx
Save the data in the netcdf file as an Excel file on average per month.

NOTE: It is improvised program for test. So code quality may not be good, there may be incorrect logic, and operation may not be guaranteed. I recommend not to use it :D

# How to use
## With cargo
`cargo run /path/to/netcdfFile.nc`

## With binary
1. Create binary with `cargo build`. then binary will be created in `target/debug/rust_netcdf`
2. Run binary `target/debug/rust_netcdf /path/to/netcdfFile.nc`
