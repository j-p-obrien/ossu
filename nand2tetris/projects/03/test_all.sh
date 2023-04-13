for i in *.tst; do
    echo $i;
    ~/ossu/nand2tetris/tools/HardwareSimulator.bat $i;
done