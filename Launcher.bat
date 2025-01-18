@echo off
cd "C:\Users\Cheuk Lab\Princeton Dropbox\Cheuk Lab\CaF\Software & Drivers\3-axis-magnetometer\magnetometer_gui"
start python gui_v3.py
cd "C:\Users\Cheuk Lab\Princeton Dropbox\Cheuk Lab\CaF\Software & Drivers\3-axis-magnetometer\3 Axis Magnetometer\target\release"
start rust_bindings.exe
exit