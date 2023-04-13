for i in *.tst; do
    echo $i;
    HardwareSimulator.sh $i;
done