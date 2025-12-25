-- tb_top.vhd
library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

-- For std.env.stop / finish (VHDL-2008)
library std;
use std.env.all;

-- OSVVM (compiled into library "osvvm")
--library osvvm;
--context osvvm.OsvvmContext;  -- pulls in the common OSVVM packages

entity tb_top is
end entity;

architecture sim of tb_top is
  constant CLK_PERIOD : time := 10 ns;

  signal clk : std_logic := '0';
  signal rst : std_logic := '1';

begin
  -- Clock
  clk <= not clk after CLK_PERIOD/2;

  -- Reset
  p_reset : process
  begin
    rst <= '1';
    wait for 5 * CLK_PERIOD;
    rst <= '0';
    wait;
  end process;

end architecture sim;
