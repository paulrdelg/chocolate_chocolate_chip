# compile the default files

# Set VIVADO_BIN_DIR to the directory which has vivado executable
set VIVADO_BIN_DIR="$XILINX_VIVADO/bin"
set XVHDL="$VIVADO_BIN_DIR/xvhdl"
set OUT_EXE="run_simulation"

# Start clean
rm -rf xsim.dir xsim.log xelab* $OUT_EXE

# compile files
$XVHDL
