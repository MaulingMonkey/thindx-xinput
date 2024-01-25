# XInput Versions

| `%THINDX_XINPUT%`                                     <br> DLL                | Ships With                                                                                <br> Minimum OS                                                                                 | API changes   |
|:-----------------------------------------------------------------------------:|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:| --------------|
| `uap`                                                 <br> XInputUap.dll      | <span style="opacity: 50%">Windows 10?</span>                                             <br> <span style="opacity: 50%">Windows 10?</span>                                              | All undocumented functions removed. <br> [On-exit bugs](https://github.com/microsoft/win32metadata/issues/1274) and COM initialization requirements added.
| `1.4`                                                 <br> XInput1_4.dll      | [Windows 10](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-versions)      <br> [Windows 8?](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-versions)       | Replaced: [get_dsound_audio_device_guids] → [get_audio_device_ids] (XAudio2.) <br> Added the undocumented: ~~[get_base_bus_information]~~, ~~[get_capabilities_ex]~~
| `1.3`                                                 <br> xinput1_3.dll      | [DirectX SDK](#directx-sdk)                                                               <br> [Windows Vista](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-versions)    | Added: [get_battery_information], [get_keystroke], + the undocumented [get_state_ex], [power_off_controller], ~~[wait_for_guide_button]~~, ~~[cancel_guide_button_wait]~~
| `1.2`                                                 <br> xinput1_2.dll      | [DirectX SDK](#directx-sdk)                                                               <br> <span style="opacity: 50%">Windows Vista?</span>                                           | <span style="opacity: 50%">APIs unchanged</span>
| `1.1`                                                 <br> xinput1_1.dll      | [DirectX SDK](#directx-sdk)                                                               <br> <span style="opacity: 50%">Windows Vista?</span>                                           | Added focus management: [enable]
| <code>9.1<span style="opacity: 50%">.0</span></code>  <br> XInput9_1_0.dll    | [Windows Vista+](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-versions)  <br> [Windows&nbsp;XP&nbsp;SP1](https://en.wikipedia.org/wiki/DirectInput#XInput)               | Base APIs: [get_state], [set_state], [get_capabilities], [get_dsound_audio_device_guids]



### DirectX SDK

Download: [DirectX SDK (June 2010)](https://www.microsoft.com/en-us/download/details.aspx?id=6812)

The DLLs can be found in `C:\Program Files (x86)\Microsoft DirectX SDK (June 2010)\Redist\*.cab`:

| DLL               | 32-bit `*.cab`            | 64-bit `*.cab`            |
| ------------------| --------------------------| --------------------------|
| `xinput1_3.dll`   | `APR2007_xinput_x86.cab`  | `APR2007_xinput_x64.cab`  |
| `xinput1_2.dll`   | `AUG2006_xinput_x86.cab`  | `AUG2006_xinput_x64.cab`  |
| `xinput1_1.dll`   | `Apr2006_xinput_x86.cab`  | `Apr2006_xinput_x64.cab`  |
| `xinput9_1_0.dll` | `Oct2005_xinput_x86.cab`  | `Oct2005_xinput_x64.cab`  |



### See Also
*   [Microsoft's Version](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-versions) of this list
