# Codex for Windows 7 x64

An unofficial Windows 7 port of OpenAI Codex **0.163.0**, based on the upstream
`rust-v0.163.0` release. This repository contains the source and compatibility
changes. The current executable package is a dev-profile build with debug
information disabled and a static CRT.

**What this build provides:** the Codex app server and its helper executables,
usable by the Codex VSCode extension or another app-server client. The small
`codex.exe` wrapper described below forwards to that server. It does **not**
provide the normal interactive terminal UI or the usual `codex exec` and
`codex login` commands.

## Files to install

Use a short, permanent folder such as `C:\Tools\CodexWin7`. Keep these files
together:

```text
C:\Tools\CodexWin7\
    codex.exe                         Existing C++ wrapper; see below
    codex-app-server.exe              Codex backend, version 0.163.0
    codex-code-mode-host.exe          JavaScript / code-mode runtime
    codex-command-runner.exe          Windows sandbox command runner
    codex-windows-sandbox-setup.exe   Windows sandbox setup helper
    codex-windows-managed-deny-probe.exe
    rg.exe                            Windows 7-compatible ripgrep
```

The local build produces the five `codex-*.exe` files in
`codex-rs/target/win7-0.163.0/`, and the transfer archive is
`codex-rs/target/codex-win7-0.163.0.zip`. These generated files are not committed
to Git. **Download ZIP** on GitHub downloads source, not the executable package.
The transfer archive does not include the existing wrapper or `rg.exe`; copy
those from the working installation, or build the wrapper below. Keep the
package's `LICENSE`, `NOTICE`, build information and checksum files too.

On the Windows 7 PC, back up the working installation and extract the new files
to a separate test folder first. Use a Windows 7-compatible x64 `rg.exe`; an
arbitrary current upstream binary may require a newer Windows version.

In Command Prompt, check the installation:

```bat
"C:\Tools\CodexWin7\codex-app-server.exe" --version
"C:\Tools\CodexWin7\codex.exe" --version
"C:\Tools\CodexWin7\rg.exe" --version
```

Both Codex version commands should report `0.163.0`. Optionally add
`C:\Tools\CodexWin7` to your user `PATH` using Windows' Environment Variables
dialog, then open a new terminal. Full paths work without changing `PATH`.

## Run without VSCode

### Start the backend

Open Command Prompt and run the server directly from your project directory:

```bat
cd /d "C:\Projects\MyProject"
"C:\Tools\CodexWin7\codex-app-server.exe" --listen stdio://
```

The process waits for newline-delimited JSON protocol messages on standard input.
It does not display a chat prompt. A quiet terminal here is expected. Standard
output carries protocol messages, and standard error carries diagnostics. Press
Ctrl+C to stop a manually started instance.

An alternative launch through the wrapper is:

```bat
"C:\Tools\CodexWin7\codex.exe" app-server --listen stdio://
```

The wrapper changes the server's working directory to the executable folder.
Clients must therefore pass the actual project path in `thread/start`'s `cwd`.
Launching the app server directly preserves the terminal's working directory.

### Connect and send a prompt

For normal use outside VSCode, use a client that implements this version of the
Codex app-server protocol. The following manual example shows the sequence; it
is not a full terminal chat client. Enter each JSON object as **one line** in
the running server's terminal, waiting for the specified response before the
next step.

1. Initialize the connection:

   ```json
   {"id":1,"method":"initialize","params":{"clientInfo":{"name":"win7_manual","version":"1.0"}}}
   ```

   After the response with `"id":1`, send:

   ```json
   {"method":"initialized","params":{}}
   ```

2. Check the saved account:

   ```json
   {"id":2,"method":"account/read","params":{}}
   ```

   If sign-in is needed, request browser sign-in:

   ```json
   {"id":3,"method":"account/login/start","params":{"type":"chatgpt"}}
   ```

   Open the returned `authUrl` in a browser on the same PC, finish signing in,
   and wait for the successful `account/login/completed` notification. If that
   browser cannot complete sign-in, device-code login lets you use a browser
   on another device when available for your account:

   ```json
   {"id":4,"method":"account/login/start","params":{"type":"chatgptDeviceCode"}}
   ```

   Follow the returned `verificationUrl` and `userCode`, then wait for completion.

3. Create a thread, replacing the project path:

   ```json
   {"id":5,"method":"thread/start","params":{"cwd":"C:\\Projects\\MyProject","sandbox":"read-only"}}
   ```

   Copy `result.thread.id` from the response with `"id":5`.

4. Replace `THREAD_ID` below with that value and send a prompt:

   ```json
   {"id":6,"method":"turn/start","params":{"threadId":"THREAD_ID","input":[{"type":"text","text":"Say hello without using tools.","text_elements":[]}]}}
   ```

   Read the streamed notifications until `turn/completed`. This step uses your
   authenticated Codex account and its usage allowance. A complete client must
   also handle server requests, including approvals; this short example only
   demonstrates a simple conversation. It starts in read-only mode.

Configuration normally lives in `%USERPROFILE%\.codex\config.toml`; account
state also lives under that Codex home. `CODEX_HOME` can select another folder.
Use the same Windows account and Codex home to share sign-in between an editor
and a standalone client. Do not publish the contents of that home directory.

If you want the familiar full-screen `codex` terminal UI, that requires a
separate Windows 7-compatible build of the main CLI. Renaming this app server
or using the wrapper does not add that interface.

## Use with VSCode 1.128.1 on Windows 7

### Get an editor that runs on Windows 7

Follow the community
[vscode-for-windows-7 1.102.1 release instructions](https://github.com/e3kskoy7wqk/vscode-for-windows-7/releases/tag/1.102.1).
Its August 2026 update reports that resource replacement also works with
**VSCode 1.128.1**:

1. Download and extract the release's Windows x64 Code - OSS archive into a
   new folder, such as `C:\Tools\VSCodeWin7`, keeping its patched runtime.
2. Download Microsoft's **Windows x64 ZIP of VSCode 1.128.1**, rather than the
   current release or installer. Extract it separately.
3. Close the editor. Back up the Code - OSS folder's existing `resources`
   directory, then replace it with `resources` from the Microsoft 1.128.1 ZIP.
   Keep the patched executable and DLLs.
4. Launch the executable in `C:\Tools\VSCodeWin7`, then verify **Help > About**
   reports 1.128.1, and open a project folder.

The project also publishes
[1.139.0](https://github.com/e3kskoy7wqk/vscode-for-windows-7/releases/tag/1.139.0).
The maintainer has not tested that newer option; these instructions target
1.128.1.

This is a community compatibility method. Microsoft's last officially supported
Windows 7 VSCode release was 1.70.3; see its
[platform FAQ](https://code.visualstudio.com/docs/supporting/faq).
This repository supplies the Codex backend, not the patched editor.

### Integrated terminal compatibility

If **Terminal > New Terminal** fails with a winpty/ConPTY error, the editor's
terminal backend needs a separate workaround. The community reports that
1.128.1 can use the older `node-pty` from VSCode 1.108. With the editor closed,
back up the target folder and replace
`resources\app\node_modules\node-pty` with the complete corresponding folder
from a separately extracted 1.108 x64 distribution. Reopen the editor and test
Command Prompt in its terminal.

This is a reported workaround, not a verified step of the Codex build. See the
[upstream issue and version-specific details](https://github.com/microsoft/vscode/issues/311849).
For later versions, including 1.139.0, the issue describes additional ASAR
packaging concerns; copying one directory may be insufficient. External Command
Prompt remains available for the standalone server instructions above.

To retain the patched installation, these User settings disable automatic
editor and extension updates:

```json
{
  "update.mode": "none",
  "extensions.autoUpdate": false
}
```

Update manually after checking compatibility, retaining a copy of the working
editor and extension. Merge settings into your existing JSON object.

### Install and configure the extension

1. Open Extensions (`Ctrl+Shift+X`) and install **Codex** from publisher
   **OpenAI**, extension ID `openai.chatgpt`. If installing offline, use
   **Extensions: Install from VSIX...** with an appropriate Windows x64 package.
   The locally inspected extension version `26.1007.21434` declares VSCode
   `^1.96.2`, which includes 1.128.1, but that declaration alone does not establish
   Windows 7 runtime compatibility. The editor's extension host and webview
   must work too.
2. Open the Command Palette (`Ctrl+Shift+P`) and select
   **Preferences: Open User Settings (JSON)**. Merge this property into the
   existing JSON object, adjusting the path:

   ```json
   {
     "chatgpt.cliExecutable": "C:\\Tools\\CodexWin7\\codex.exe"
   }
   ```

   Use **User** settings. This is an application-scoped extension setting;
   it does not belong in `config.toml`. Point it at the **wrapper** so the
   extension's `app-server` argument is handled correctly. OpenAI labels this
   override as a development setting; it is used here to select the custom port.
   See [official extension settings](https://learn.chatgpt.com/docs/ide/settings).
3. Run **Developer: Reload Window**, open your project folder, and open the
   Codex sidebar. Let the extension launch the backend; you do not need to
   start a separate stdio server in a terminal.
4. Sign in through the sidebar. Test a short conversation, then a file read,
   command execution with your existing sandbox configuration, and code-mode
   execution. Check that edits apply to the intended project folder.

Keeping the custom executables outside the extension's installation directory
lets the path remain stable across extension updates. If an extension update
changes the backend protocol, retain or reinstall the known working extension
version while checking compatibility with this 0.163.0 backend.

## How the wrapper works

`codex.exe` is a small C++ console program, separate from the Rust executables.
It finds `codex-app-server.exe` beside itself, removes the exact arguments
`app-server` and `--analytics-default-enabled`, and forwards the remaining
arguments with Windows command-line quoting. It inherits stdin/stdout/stderr,
waits for the server, and returns its exit code. It starts the server with the
executable folder as its working directory.

This makes the standalone app-server executable fit the extension's expected
`codex app-server ...` launch command. It does not implement login, exec, or
terminal UI subcommands itself. Keep the filename `codex-app-server.exe`
unchanged and use a short installation path; this wrapper uses `MAX_PATH`.

The existing wrapper is not included in the Rust build or transfer archive.
Its source is reproduced below so it can be rebuilt independently. On a build
PC with Visual C++ tools, save it as `codex_wrapper.cpp`, open an **x64 Native
Tools Command Prompt**, and compile a console executable:

```bat
cl /nologo /EHsc /O2 /MT /D_WIN32_WINNT=0x0601 codex_wrapper.cpp /link /SUBSYSTEM:CONSOLE,6.01 /OUT:codex.exe
```

Use a toolchain whose generated binaries support Windows 7, then test the result
there. `/MT` selects the static release C runtime. This is a command-line build
recipe; the original Visual Studio project has separate Debug/Release settings.

## Troubleshooting and validation

| Symptom | What to check |
| --- | --- |
| Server starts but displays no prompt | Expected for app-server: connect a protocol client or use the JSON sequence above. |
| `unexpected argument 'app-server'` | The extension is calling the server directly. Set `chatgpt.cliExecutable` to the wrapper. |
| Wrapper exits immediately | Confirm `codex-app-server.exe` is beside it, then run that executable directly to see its error. |
| Missing DLL or entry point on Windows 7 | Check that you copied the custom Windows 7 binaries and compatible wrapper/`rg.exe`, rather than stock current Windows builds. |
| Codex fails only inside the editor | Check **View > Output** for Codex diagnostics and **Developer: Show Logs... > Extension Host**, then verify the executable setting and extension version. |
| Tasks use the installation folder | Supply the project `cwd` in a standalone client's thread request; the wrapper resets the process working directory. |
| Browser sign-in fails | Check the system clock, HTTPS connectivity and certificates; try device-code login when available. |
| Sandbox setup fails | Capture the error and check the existing Win7 sandbox configuration. The newer WinRT/MXC isolation path is unavailable on Windows 7. |

Build-host checks passed for version reporting, app-server initialization,
code-mode JavaScript execution, four focused Windows firewall tests, and PE
import screening for known newer-Windows dependencies. The build host was
Windows 10. Actual Windows 7 execution and the complete VSCode 1.128.1 integration
still require verification on the destination PC.

## Upstream and license

- [OpenAI Codex source](https://github.com/openai/codex)
- [Codex documentation](https://developers.openai.com/codex)
- [Apache-2.0 license](LICENSE) and [notices](NOTICE)


<details>
<summary>Wrapper source: codex_wrapper.cpp</summary>

```cpp
#define _CRT_SECURE_NO_WARNINGS
#define UNICODE
#define _UNICODE

#include <windows.h>
#include <string>
#include <vector>

static std::wstring QuoteArg(const std::wstring& arg)
{
    // No quoting required if there are no spaces, tabs or quotes.
    if (arg.find_first_of(L" \t\"") == std::wstring::npos)
        return arg;

    std::wstring result = L"\"";
    unsigned int backslashes = 0;

    for (size_t i = 0; i < arg.length(); ++i)
    {
        wchar_t ch = arg[i];

        if (ch == L'\\')
        {
            ++backslashes;
        }
        else if (ch == L'"')
        {
            // Escape all preceding backslashes, then escape the quote.
            result.append(backslashes * 2 + 1, L'\\');
            result += L'"';
            backslashes = 0;
        }
        else
        {
            result.append(backslashes, L'\\');
            backslashes = 0;
            result += ch;
        }
    }

    // Backslashes immediately before the closing quote must be doubled.
    result.append(backslashes * 2, L'\\');
    result += L'"';

    return result;
}

int wmain(int argc, wchar_t* argv[])
{
    wchar_t modulePath[MAX_PATH];

    DWORD len = GetModuleFileNameW(
        NULL,
        modulePath,
        sizeof(modulePath) / sizeof(modulePath[0])
    );

    if (len == 0 || len >= MAX_PATH)
        return 1;

    std::wstring exePath(modulePath, len);

    size_t slash = exePath.find_last_of(L"\\/");
    if (slash == std::wstring::npos)
        return 1;

    std::wstring directory = exePath.substr(0, slash + 1);
    std::wstring appServerPath = directory + L"codex-app-server.exe";

    std::wstring commandLine = QuoteArg(appServerPath);

    for (int i = 1; i < argc; ++i)
    {
        std::wstring arg = argv[i];

        // These arguments belong to the main Codex CLI layer and must not
        // be passed to the standalone codex-app-server executable.
        if (arg == L"app-server")
            continue;

        if (arg == L"--analytics-default-enabled")
            continue;

        commandLine += L" ";
        commandLine += QuoteArg(arg);
    }

    // CreateProcessW requires a writable command-line buffer.
    std::vector<wchar_t> cmd(
        commandLine.begin(),
        commandLine.end()
    );
    cmd.push_back(L'\0');

    STARTUPINFOW si;
    PROCESS_INFORMATION pi;

    ZeroMemory(&si, sizeof(si));
    ZeroMemory(&pi, sizeof(pi));

    si.cb = sizeof(si);

    BOOL ok = CreateProcessW(
        appServerPath.c_str(),
        &cmd[0],
        NULL,
        NULL,
        TRUE,       // inherit stdin/stdout/stderr
        0,
        NULL,
        directory.c_str(),
        &si,
        &pi
    );

    if (!ok)
        return (int)GetLastError();

    WaitForSingleObject(pi.hProcess, INFINITE);

    DWORD exitCode = 1;
    GetExitCodeProcess(pi.hProcess, &exitCode);

    CloseHandle(pi.hThread);
    CloseHandle(pi.hProcess);

    return (int)exitCode;
}
```

</details>
