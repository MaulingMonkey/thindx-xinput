use crate::*;
use winapi_0_2::guiddef::GUID;
use winapi_0_2::xinput::*;



struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, FromInto })]
    BatteryInformation => XINPUT_BATTERY_INFORMATION {
        battery_type    => BatteryType,
        battery_level   => BatteryLevel,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Capabilities => XINPUT_CAPABILITIES {
        ty          => Type,
        sub_type    => SubType,
        flags       => Flags,
        gamepad     => Gamepad,
        vibration   => Vibration,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Gamepad => XINPUT_GAMEPAD {
        buttons         => wButtons,
        left_trigger    => bLeftTrigger,
        right_trigger   => bRightTrigger,
        #[renamed] left_thumb_x    => sThumbLX,
        #[renamed] left_thumb_y    => sThumbLY,
        #[renamed] right_thumb_x   => sThumbRX,
        #[renamed] right_thumb_y   => sThumbRY,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    #[renamed] DSoundAudioDeviceGuid => GUID {
        // fields not individually defined
    }

    // `winapi = "0.2.8"` has a broken definition of `XINPUT_KEYSTROKE` which is missing a `Flags` field.
    //
    // This field is never missing in XInput:
    // • XInput 1.4 (verified in `C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\Xinput.h`   which has `XINPUT_DLL_A = "xinput1_4.dll"`)
    // • XInput 1.3 (verified in `C:\Program Files (x86)\Microsoft DirectX SDK (June 2010)\Include\XInput.h` which has `XINPUT_DLL_A = "xinput1_3.dll"`)
    // • XInput 1.2 and earlier does not define the keystroke API, including this struct.
    // • XInput UAP presumably is equivalent to XInput 1.4
    //
    // While `FromInto` could still be implemented, any actual use of winapi 0.2's XINPUT_KEYSTROKE is almost certainly a bug, and so I've instead chosen to discourage it's use.
    //
    //#[derive(unsafe { AsRef, AsMut, FromInto })]
    //Keystroke => XINPUT_KEYSTROKE {}

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    State => XINPUT_STATE {
        packet_number   => dwPacketNumber,
        gamepad         => Gamepad,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Vibration => XINPUT_VIBRATION {
        left_motor_speed    => wLeftMotorSpeed,
        right_motor_speed   => wRightMotorSpeed,
    }
}
