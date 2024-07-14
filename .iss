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
Filename: "{app}\oguard.exe"; Parameters: "service create"; StatusMsg: "Installing service..."; Flags: runhidden waituntilterminated

; Start the service
[Run]
Filename: "{app}\oguard.exe"; Parameters: "service start"; StatusMsg: "Starting service..."; Flags: runhidden waituntilterminated

; Define the uninstall procedure
[UninstallRun]
Filename: "{app}\oguard.exe"; Parameters: "service stop"; StatusMsg: "Stopping service..."; Flags: runhidden waituntilterminated
Filename: "{app}\oguard.exe"; Parameters: "service delete"; StatusMsg: "Deleting service..."; Flags: runhidden waituntilterminated

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