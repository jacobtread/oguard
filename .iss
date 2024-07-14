; Define your application information
[Setup]
AppName=OGuard
AppVersion=1.0
DefaultDirName={pf}\OGuard
DefaultGroupName=OGuard
OutputDir=.
OutputBaseFilename=setup
Compression=lzma
SolidCompression=yes

; Specify the application files
[Files]
Source: ".\target\release\oguard.exe"; DestDir: "{app}"; Flags: ignoreversion

; Create the service
[Run]
Filename: "sc.exe"; Parameters: "create oguard start= auto binPath= ""{app}\oguard.exe"""; StatusMsg: "Installing service..."; Flags: runhidden waituntilterminated

; Setup service working directory
[Run]
Filename: "sc.exe"; Parameters: "config oguardWh obj= LocalSystem depend= """" start= auto AppDirectory= ""{app}"""; Flags: runhidden

; Start the service
[Run]
Filename: "sc.exe"; Parameters: "start oguard"; StatusMsg: "Starting service..."; Flags: runhidden waituntilterminated

; Define the uninstall procedure
[UninstallRun]
Filename: "sc.exe"; Parameters: "stop oguard"; StatusMsg: "Stopping service..."; Flags: runhidden waituntilterminated
Filename: "sc.exe"; Parameters: "delete oguard"; StatusMsg: "Deleting service..."; Flags: runhidden waituntilterminated

; Optional: define the messages for the user
[Messages]
SetupMessage=OGuard service is being installed...
UninstallMessage=OGuard service is being uninstalled...


[Code]
 procedure CurUninstallStepChanged (CurUninstallStep: TUninstallStep);
 var
     mres : integer;
 begin
    case CurUninstallStep of                   
      usPostUninstall:
        begin
          mres := MsgBox('Do you want to delete the configuration file, data folder and its contents? This will delete any historical data, logging and configuration?', mbConfirmation, MB_YESNO or MB_DEFBUTTON2)
          if mres = IDYES then
            DelTree(ExpandConstant('{app}\data'), True, True, True);
            DeleteFile(ExpandConstant('{app}\config.toml'));
       end;
   end;
end;