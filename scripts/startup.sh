#source /c/AMDDesignTools/2025.2/Vivado/settings64.sh


# Set VIVADO_BIN_DIR to the directory which has vivado executable

#set VIVADO_BIN_DIR="$RDI_ROOT/prep/rdi/vivado/bin"
set VIVADO_BIN_DIR="$XILINX_VIVADO/bin"

set VIVADO_DATA_DIR="$VIVADO_BIN_DIR/../data"
set OUT_SIM_SNAPSHOT="adder"
set XSI_INCLUDE_DIR="$VIVADO_DATA_DIR/xsim/include"
set GCC_COMPILER="/usr/bin/g++"
set XSIM_ELAB="xelab"
set OUT_EXE="run_simulation"

# Start clean
rm -rf xsim.dir xsim.log xelab* $OUT_EXE

#$XSIM_ELAB work.adder_vhdl -prj adder.prj -dll -s $OUT_SIM_SNAPSHOT

set XVHDL="$VIVADO_BIN_DIR/xvhdl"

$XVHDL --version
