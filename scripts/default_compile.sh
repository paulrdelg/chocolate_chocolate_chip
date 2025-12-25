# compile the default files

# Set VIVADO_BIN_DIR to the directory which has vivado executable
set VIVADO_BIN_DIR="$XILINX_VIVADO/bin"
set XVHDL="$VIVADO_BIN_DIR/xvhdl"
set XELAB="$VIVADO_BIN_DIR/xelab"
set OUT_EXE="run_simulation"

# Start clean
echo "cleaning..."
rm -rf xsim.dir xsim.log xelab* $OUT_EXE

# compile files
echo "compiling..."

mkdir -p libs/tb_lib
$XVHDL "./src/firmware/top_tb.vhd" -work tb_lib=./libs/tb_lib

mkdir -p libs/dut_lib
$XVHDL -2008 -work dut_lib=./libs/tb_lib ./src/firmware/dut.vhd

# finished
echo "finished compilation"

# elaborate
echo "elaborating..."

$XELAB -L tb_lib work.tb_top

# finished
echo "finished elaboration"
