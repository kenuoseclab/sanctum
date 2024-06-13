# Sanctum Driver

This is the driver module of the EDR.

For the full list of features etc, see my [blog](https://fluxsec.red/sanctum-edr-intro).

## Development cheat sheet:

### Building 

In the EWDK terminal (admin) navigate to the driver directory and build with `cargo make`.

You can use `cargo build` and `cargo check` to check the code; but you need to do make to build the sys file.

### Deployment 

To deploy it to the testing machine:

1) Run `cargo make` as above
2) Copy the .sys file into the VM to a place of your choosing
3) In an elevated powershell session, run: 
   1) `sc.exe stop Sanctum`
   2) `sc.exe delete Sanctum`
   3) `sc.exe create Sanctum binPath= "C:\Drivers\sanctum_driver_package\sanctum_driver.sys" type= kernel`
   4) `sc.exe start Sanctum`

### Debugging:

 - For setting symbol paths:
   - `.sympath srv*C:\Symbols*https://msdl.microsoft.com/download/symbols`
   - `.reload /f`
 - Set a breakpoint at the entry: `bp sanctum_driver!DriverEntry`
 - Check debug output with `!dbgprint`
 - If any problems no debug printing, try `ed Kd_DEFAULT_Mask 8`
 - `lm` to show loaded modules

### Other

 - To view the status of the driver in the VM: 
   - `Get-Service -Name Sanctum`
   - `sc.exe qc Sanctum`